pub mod renderer;
pub mod texture;
pub use wgpu::SwapChainError;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
