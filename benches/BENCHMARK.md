## Benchmark Results

### Color Legend

- 🟩 **Green**: Best performance (minimum value) or within 50% of the best
- 🟨 **Yellow**: Moderate performance (up to 2x the minimum value)
- 🟥 **Red**: Poor performance (more than 2x the minimum value)

### CU Consumed

| Benchmark     | `pinocchio`     | `anchor`          | `typhoon`    | `star-frame`   |
| ------------- | --------------- | ----------------- | ------------ | -------------- |
| ping | 🟩 12 (+1) | 🟥 238 (+227) | 🟩 **11** | 🟩 13 (+2) |
| log | 🟩 **116** | 🟥 342 (+226) | 🟩 117 (+1) | 🟩 117 (+1) |
| create_account | 🟩 1570 (+113) | 🟥 3790 (+2333) | 🟩 **1457** | 🟩 1550 (+93) |
| transfer | 🟩 **1289** | 🟨 2442 (+1153) | 🟩 1300 (+11) | 🟩 1316 (+27) |
| unchecked_accounts | 🟩 **99** | 🟥 1738 (+1639) | 🟩 100 (+1) | 🟩 105 (+6) |
| accounts | 🟩 316 (+24) | 🟥 1711 (+1419) | 🟩 **292** | 🟩 358 (+66) |

### Binary Size

|                     | `pinocchio`     | `anchor`            | `typhoon`| `star-frame`   |
| ------------------- | --------------- | ------------------- | -------- | -------------- |
| Binary size (bytes) | 🟩 17944 (+2488) | 🟥 212560 (+197104) | 🟩 **15456** | 🟥 115264 (+99808) |
