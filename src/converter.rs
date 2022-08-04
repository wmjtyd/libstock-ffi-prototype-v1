pub mod decimal;

/// ConvertBox 是給 foreign type 進行轉換的 0 成本 wrapper。
///
/// 如果是同類型、或可以直接進行 into 的類型的話，就不必使用了。
pub struct ConvertBox<T>(pub T);

impl<T> std::ops::Deref for ConvertBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for ConvertBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
