# HashMap

- [HashMap](#hashmap)
  - [简介](#简介)
  - [创建 HashMap](#创建-hashmap)
    - [使用 new 创建](#使用-new-创建)
    - [从其它集合创建](#从其它集合创建)
  - [所有权转移](#所有权转移)
  - [查询 HashMap](#查询-hashmap)
  - [更新 HashMap 值](#更新-hashmap-值)
    - [在已有值上更新](#在已有值上更新)
  - [哈希函数](#哈希函数)

2023-10-27, 10:16
@author Jiawei Mao
****

## 简介

`HashMap` 存储键值对，提供平均复杂度为 `O(1)` 的查询方法。

## 创建 HashMap

### 使用 new 创建

```rust
use std::collections::HashMap;

// 创建一个HashMap，用于存储宝石种类和对应的数量
let mut my_gems = HashMap::new();

// 将宝石类型和对应的数量写入表中
my_gems.insert("红宝石", 1);
my_gems.insert("蓝宝石", 2);
my_gems.insert("河边捡的误以为是宝石的破石头", 18);
```

该 HashMap 类型为 `HashMap<&str,i32>`。

跟 `Vec` 一样，如果预先知道要存储的 KV 对个数，可以使用 `HashMap::with_capacity(capacity)` 创建指定大小的 `HashMap` ，避免频繁的内存分配和拷贝，提升性能。

### 从其它集合创建

- 从 Vec 创建 HashMap

```rust
fn main() {
    use std::collections::HashMap;

    let teams_list = vec![
        ("中国队".to_string(), 100),
        ("美国队".to_string(), 10),
        ("日本队".to_string(), 50),
    ];

    let mut teams_map = HashMap::new();
    for team in &teams_list {
        teams_map.insert(&team.0, team.1);
    }

    println!("{:?}",teams_map)
}
```

即遍历 Vec，然后依次插入 `HashMap`。不过该方法略显笨拙。

- 迭代器方法

```rust
fn main() {
    use std::collections::HashMap;

    let teams_list = vec![
        ("中国队".to_string(), 100),
        ("美国队".to_string(), 10),
        ("日本队".to_string(), 50),
    ];

    let teams_map: HashMap<_,_> = teams_list.into_iter().collect();
    
    println!("{:?}",teams_map)
}
```

`into_iter()` 将列表转换为迭代器，然后通过 `collect` 收集。`collect` 支持生成多种类型的集合，因此需要通过类型标注 `HashMap<_,_>` 告诉编译器转换为 `HashMap` 集合类型，而 `KV` 类型由编译器推导。

## 所有权转移

`HashMap` 的所有权规则与其它 Rust 类型相同：

- 若类型实现 `Copy` 特征，该类型会被复制进 `HashMap` ，因此无所谓所有权
- 若没实现 `Copy` 特征，所有权将被转移到 `HashMap`

**示例：**

```rust
fn main() {
    use std::collections::HashMap;

    let name = String::from("Sunface");
    let age = 18;

    let mut handsome_boys = HashMap::new();
    handsome_boys.insert(name, age);

    println!("因为过于无耻，{}已经被从帅气男孩名单中除名", name);
    println!("还有，他的真实年龄远远不止{}岁", age);
}
```

运行报错：

```sh
error[E0382]: borrow of moved value: `name`
  --> src/main.rs:10:32
   |
4  |     let name = String::from("Sunface");
   |         ---- move occurs because `name` has type `String`, which does not implement the `Copy` trait
...
8  |     handsome_boys.insert(name, age);
   |                          ---- value moved here
9  |
10 |     println!("因为过于无耻，{}已经被除名", name);
   |                                            ^^^^ value borrowed here after move
```

提示 `name` 是 `String` 类型，因此它受到所有权的限制，在 `insert` 时，它的所有权转移
给 `handsome_boys`，所以最后在使用时会报错。

如果将**引用类型**放入 `HashMap`，需确保该引用的生命周期至少不低于 HashMap：

```rust
fn main() {
    use std::collections::HashMap;

    let name = String::from("Sunface");
    let age = 18;

    let mut handsome_boys = HashMap::new();
    handsome_boys.insert(&name, age);

    std::mem::drop(name);
    println!("因为过于无耻，{:?}已经被除名", handsome_boys);
    println!("还有，他的真实年龄远远不止{}岁", age);
}
```

上面借用 `name` 获取了它的引用，然后插入到 `handsome_boys`，至此一切都很完美。但是紧接着通过 `drop` 函数手动将 `name` 字符串从内存中移除，因此报错：

```sh
 handsome_boys.insert(&name, age);
   |                          ----- borrow of `name` occurs here // name借用发生在此处
9  |
10 |     std::mem::drop(name);
   |                    ^^^^ move out of `name` occurs here // name的所有权被转移走
11 |     println!("因为过于无耻，{:?}已经被除名", handsome_boys);
   |                                              ------------- borrow later used here // 所有权转移后，还试图使用name
```

## 查询 HashMap

通过 `get` 方法获取元素：

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score: Option<&i32> = scores.get(&team_name);
```

要点：

- `get` 方法返回 `Option<&i32>` 类型：当查询不到时返回 `None`，查询到时返回 `Some(&i32)`
- `&i32` 是对 `HashMap` 中值的借用，如果不使用借用，可能会发生所有权的转移

如果想直接获得值类型的 `score` 该怎么办，答案简约但不简单:

```rust
let score: i32 = scores.get(&team_name).copied().unwrap_or(0);
```

还可以通过循环遍历 `KV` 对：

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```

```sh
Yellow: 50
Blue: 10
```

## 更新 HashMap 值

示例：

```rust
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert("Blue", 10);

    // 覆盖已有的值
    let old = scores.insert("Blue", 20);
    assert_eq!(old, Some(10));

    // 查询新插入的值
    let new = scores.get("Blue");
    assert_eq!(new, Some(&20));

    // 查询Yellow对应的值，若不存在则插入新值
    let v = scores.entry("Yellow").or_insert(5);
    assert_eq!(*v, 5); // 不存在，插入5

    // 查询Yellow对应的值，若不存在则插入新值
    let v = scores.entry("Yellow").or_insert(50);
    assert_eq!(*v, 5); // 已经存在，因此50没有插入
}
```

### 在已有值上更新

查询某个 key 对应的值，若不存在则插入新值，若存在则对已有的值进行更新，例如在文本中统计词语出现的次数：

```rust
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();
// 根据空格来切分字符串(英文单词都是通过空格切分)
for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{:?}", map);
```

上面新建一个 `map` 用于保存词语出现的次数，插入一个词语时会进行判断：若之前没有插入过，则使用该词语作 `Key` ，插入次数 0 作为 `Value` ，若之前插入过则取出之前统计的该词语出现的次数，对其加一。

有两点值得注意：

- `or_insert` 返回 `&mut v` 引用，因此可以通过该可变引用直接修改 `map` 中对应的值
- 使用 `count` 引用时，需要先进行解引用 *count ，否则会出现类型不匹配

## 哈希函数

一个类型能够作为 HashMap 的 `Key` 的关键时能够进行想等性比较，即是否实现了 `std::cmp::Eq`。

!!! attention
    f32 和 f64 没有实现 `std::cmp::Eq` 特征，因此不能用作 HashMap 的 Key。

目前 HashMap 使用的哈希函数是 `SipHash`，它的性能不高，但是很安全。SipHash 在中等大小的 Key 上，性能很好，但是对小型 Key （如整数）或者大型 Key （如字符串），性能不够好。如果需要更好的性能，可以考虑 ahash 这个库。

第三方库使用：

```rust
use std::hash::BuildHasherDefault;
use std::collections::HashMap;
// 引入第三方的哈希函数
use twox_hash::XxHash64;

// 指定HashMap使用第三方的哈希函数XxHash64
let mut hash: HashMap<_, _, BuildHasherDefault<XxHash64>> = Default::default();
hash.insert(42, "the answer");
assert_eq!(hash.get(&42), Some(&"the answer"));
```
