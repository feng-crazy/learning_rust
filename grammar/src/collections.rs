pub(crate) fn collections(){
    // 用宏创建可变向量
    let mut v1 = vec![];
    v1.push(1);
    v1.push(2);
    v1.push(3);
    assert_eq!(v1, [1,2,3]);
    assert_eq!(v1[1], 2);

// 用宏创建不可变向量
    let v2 = vec![0; 10];
    println!("v2: {:?}", v2);
    for i in v2 {
        println!("v2 i: {}", i);
    }
// 用 new 方法创建
    let mut v3 = Vec::new();
    v3.push(4);
    v3.push(5);
    v3.push(6);


    use std::collections::BTreeMap;
    use std::collections::HashMap;
    let mut hmap = HashMap::new();
    let mut bmap = BTreeMap::new();
    hmap.insert(3, "c");
    hmap.insert(1, "a");
    hmap.insert(2, "b");
    hmap.insert(5, "e");
    hmap.insert(4, "d");
    // hmap.insert("4", "f");
    bmap.insert(3, "c");
    bmap.insert(2, "b");
    bmap.insert(1, "a");
    bmap.insert(5, "e");
    bmap.insert(4, "d");
// 输出结果为：{1: "a", 2: "b", 3: "c", 5: "e", 4: "d"}，但key的顺序是随机的，因为HashMap是无序的
    println!("{:?}", hmap);
// 输出结果永远都是 {1: "a", 2: "b", 3: "c", 4: "d", 5: "e"}，因为BTreeMap是有序的
    println!("{:?}", bmap);


    use std::collections::HashSet;
    use std::collections::BTreeSet;
    let mut hbooks = HashSet::new();
    let mut bbooks = BTreeSet::new();
// 插入数据
    hbooks.insert(2);
    hbooks.insert(1);
    hbooks.insert(2);
// 判断元素是否存在，contains方法和HashMap中的一样
    if !hbooks.contains(&1) {
    }
    println!("{:?}", hbooks);
    bbooks.insert(1);

    bbooks.insert(2);
    bbooks.insert(3);
    println!("{:?}", bbooks); // 输出固定为 {1, 2, 3} ，因为是有序


    use std::collections::BinaryHeap;
    let mut heap = BinaryHeap::new();
    assert_eq!(heap.peek(), None);
    heap.push(93);
    heap.push(80);
    heap.push(48);
    heap.push(53);
    heap.push(72);
    heap.push(30);
    heap.push(18);
    heap.push(36);
    heap.push(15);
    heap.push(35);
    heap.push(45);
    assert_eq!(heap.peek(), Some(&93));
    println!("{:?}", heap);  // [93, 80, 48, 53, 72, 30, 18, 36, 15, 35, 45]
}