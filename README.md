# Libstock 的 FFI

## 編譯說明

使用 `Makefile` 進行編譯。

```bash
# 產生優化過的執行檔案。
make dist

# 產生除錯用的執行檔案。
make dist-debug

# 清理。
make clean

# (will be deprecated) 編譯測試用例。
make exp
```

## 簽名慣例

> WIP; may change in the future!

```c
typedef uint32_t return_type;

// 分配儲存 `<structure>` 所需的空間。
<structure>* new_<structure>(void);

// 釋放 new_<structure> 分配的空間。
void free_<structure>(<structure>* s);

// 將傳入的 <structure> 進行序列化，並儲存至指定空間。
return_type serialize_<structure>(*<structure> structure, *uint8_t slice, *size_t written_size);

// 將傳入的 <structure> 進行反序列化，並儲存至分配的空間。
return_type deserialize_<structure>(*uint8_t slice_ref, *<structure> target);
```
