# RCuber
Rust Cuber, 一个Rust版本的魔方(Rubick's Cube)库，支持终端展示和一系列的解法（CFOP、LBL、Roux、Min2Phase）。

### 基本设计
1. [kociemba](https://crates.io/crates/kociemba)的基础魔方库（`CubieCube`,`FaceCube`,`Move`,`Generator`）
2. LBL（层先、入门）算法
3. CFOP算法（移植 https://pypi.org/project/pycuber/）
4. min2phase算法（移植 https://github.com/cs0x7f/min2phase）
5. Roux（桥式）算法（参考 https://github.com/onionhoney/roux-trainers）

### Crates.io
* https://crates.io/crates/rcuber
### github
* https://github.com/adungaos/rcuber
### TODO
* Roux（桥式）解法的效率（XXPruner的max_depth值：较小则初始化时间小，整体解法时间小，但某些情况下出现较大的长尾现象；较大则solve时间小，解法时间偏差小，需要选择合适的值达到平衡。
  * LB: 4 vs 5
  * SB: 7 vs 6
  * LSE：6 vs 5
* Roux相关代码重构，去除重复代码。
* 注释和代码清理。

### 参考资料
* [kociemba](https://crates.io/crates/kociemba)
* [pycuber](https://pypi.org/project/pycuber/)
* [min2phase](https://github.com/cs0x7f/min2phase)
* [Roux Trainers](https://github.com/onionhoney/roux-trainers)
* [Kewb](https://github.com/luckasRanarison/kewb)
* 文档：各种解法的说明来自[seppedsolving.com](https://www.speedsolving.com/wiki/index.php?title=Main_Page)


------

# English
