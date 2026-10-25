use crate::{config::Config, models::ToolInfo};
use anyhow::{anyhow, Result};
use std::path::PathBuf;
use tokio::process::Command;

#[derive(Clone)]
pub struct ToolRegistry { config: Config }

impl ToolRegistry {
    pub fn new(config: Config) -> Self { Self { config } }

    pub fn list(&self) -> Vec<ToolInfo> {
        vec![
            ToolInfo { name:"calculator".into(), description:"Evaluate basic arithmetic (+,-,*,/).".into(), enabled:true },
            ToolInfo { name:"read_file".into(), description:"Read UTF-8 text inside the workspace.".into(), enabled:true },
            ToolInfo { name:"write_file".into(), description:"Write UTF-8 text inside the workspace.".into(), enabled:self.config.allow_file_write },
            ToolInfo { name:"shell".into(), description:"Run a shell command when explicitly enabled.".into(), enabled:self.config.allow_shell },
        ]
    }

    pub fn calculate(&self, input: &str) -> Result<f64> {
        let tokens = tokenize(input)?;
        let mut values: Vec<f64> = Vec::new();
        let mut ops: Vec<char> = Vec::new();
        for token in tokens {
            match token {
                Token::Number(n) => values.push(n),
                Token::Op(op) => {
                    while ops.last().is_some_and(|top| precedence(*top) >= precedence(op)) {
                        apply(&mut values, ops.pop().unwrap())?;
                    }
                    ops.push(op);
                }
            }
        }
        while let Some(op) = ops.pop() { apply(&mut values, op)?; }
        if values.len() != 1 { return Err(anyhow!("invalid arithmetic expression")); }
        Ok(values[0])
    }

    pub async fn read_file(&self, relative: &str) -> Result<String> {
        let path = self.safe_path(relative)?;
        Ok(tokio::fs::read_to_string(path).await?)
    }

    pub async fn write_file(&self, relative: &str, content: &str) -> Result<()> {
        if !self.config.allow_file_write { return Err(anyhow!("file writing is disabled")); }
        let path = self.safe_path_for_write(relative).await?;
        if let Some(parent) = path.parent() { tokio::fs::create_dir_all(parent).await?; }
        tokio::fs::write(path, content).await?;
        Ok(())
    }

    pub async fn run_shell(&self, command: &str) -> Result<String> {
        if !self.config.allow_shell { return Err(anyhow!("shell execution is disabled")); }
        if command.trim().is_empty() { return Err(anyhow!("empty command")); }
        #[cfg(target_os="windows")]
        let output = Command::new("cmd").args(["/C", command]).output().await?;
        #[cfg(not(target_os="windows"))]
        let output = Command::new("sh").args(["-lc", command]).output().await?;
        Ok(format!("exit_code={}\nstdout:\n{}\nstderr:\n{}",
            output.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)))
    }

    fn root(&self) -> Result<PathBuf> { Ok(std::fs::canonicalize(&self.config.workspace)?) }

    fn safe_path(&self, relative: &str) -> Result<PathBuf> {
        let root = self.root()?;
        let actual = std::fs::canonicalize(root.join(relative))?;
        if !actual.starts_with(&root) { return Err(anyhow!("path escapes workspace")); }
        Ok(actual)
    }

    async fn safe_path_for_write(&self, relative: &str) -> Result<PathBuf> {
        let root = self.root()?;
        let candidate = root.join(relative);
        if candidate.exists() {
            let actual = std::fs::canonicalize(&candidate)?;
            if !actual.starts_with(&root) { return Err(anyhow!("path escapes workspace")); }
            return Ok(actual);
        }
        let parent = candidate.parent().ok_or_else(|| anyhow!("invalid path"))?;
        tokio::fs::create_dir_all(parent).await?;
        let parent = std::fs::canonicalize(parent)?;
        if !parent.starts_with(&root) { return Err(anyhow!("path escapes workspace")); }
        Ok(parent.join(candidate.file_name().ok_or_else(|| anyhow!("invalid filename"))?))
    }
}

enum Token { Number(f64), Op(char) }
fn tokenize(input: &str) -> Result<Vec<Token>> {
    let mut out = Vec::new();
    let mut number = String::new();
    let flush = |number: &mut String, out: &mut Vec<Token>| -> Result<()> {
        if !number.is_empty() {
            out.push(Token::Number(number.parse().map_err(|_| anyhow!("invalid number"))?));
            number.clear();
        }
        Ok(())
    };
    for ch in input.chars() {
        if ch.is_ascii_digit() || ch == '.' { number.push(ch); }
        else if "+-*/".contains(ch) { flush(&mut number,&mut out)?; out.push(Token::Op(ch)); }
        else if ch.is_whitespace() { flush(&mut number,&mut out)?; }
        else { return Err(anyhow!("unsupported character: {ch}")); }
    }
    flush(&mut number,&mut out)?;
    Ok(out)
}
fn precedence(op: char) -> u8 { match op { '+'|'-'=>1, '*'|'/'=>2, _=>0 } }
fn apply(values: &mut Vec<f64>, op: char) -> Result<()> {
    if values.len()<2 { return Err(anyhow!("operator missing operands")); }
    let b=values.pop().unwrap(); let a=values.pop().unwrap();
    let result=match op {
        '+'=>a+b, '-'=>a-b, '*'=>a*b,
        '/'=> { if b==0.0 { return Err(anyhow!("division by zero")); } a/b },
        _=>return Err(anyhow!("unsupported operator"))
    };
    values.push(result); Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn arithmetic_precedence() {
        let cfg=Config{host:"127.0.0.1".into(),port:8080,database_url:"sqlite::memory:".into(),ollama_base_url:"http://localhost:11434".into(),ollama_model:"test".into(),system_prompt:"test".into(),workspace:".".into(),allow_shell:false,allow_file_write:false};
        assert_eq!(ToolRegistry::new(cfg).calculate("2 + 3 * 4").unwrap(),14.0);
    }
    #[test] fn division_by_zero_rejected() {
        let cfg=Config{host:"127.0.0.1".into(),port:8080,database_url:"sqlite::memory:".into(),ollama_base_url:"http://localhost:11434".into(),ollama_model:"test".into(),system_prompt:"test".into(),workspace:".".into(),allow_shell:false,allow_file_write:false};
        assert!(ToolRegistry::new(cfg).calculate("1 / 0").is_err());
    }
}
