use error::ConnectError;
use tokio::io::AsyncWriteExt;
use tokio::io::WriteHalf;
use tokio::net::TcpStream;

pub mod error;

pub async fn handle_unknown() -> Result<(), ConnectError> {
    Ok(())
}

pub async fn handle_connect_cmd(stream: &mut WriteHalf<TcpStream>) -> Result<(), ConnectError> {
    stream
        .write_all("#bs connect_resp\n#end\n".as_bytes())
        .await?;
    stream.flush().await?;
    Ok(())
}
