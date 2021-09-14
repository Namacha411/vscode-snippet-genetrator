# vscode-snippet-genetrator

this is visual studio code user snippet generator.

コマンドを実行したディレクトリの下にあるファイルからvisual studio codeのユーザースニペットを生成します。

競技プログラミングなどを行う際に、自前のライブラリを作っていた場合スニペットを生成することでライブラリからコピペする手間を省くことができます。

## 使い方

ユーザースニペット化したいディレクトリで拡張子を指定してコマンドを実行します。

```sh
snippet_generator <expansion>
```

生成されたcode-snippetをvisual studio codeのユーザースニペットとして登録してください。

## 例

以下のように生成されます。

```sh
$ snippet_generator rs
{
  "lib.rs": {
    "prefix": "lib.rs",
    "body": [
      "mod dijkstra;",
      "",
      "#[cfg(test)]",
      "mod tests {",
      "    #[test]",
      "    fn it_works() {",
      "    }",
      "}"
    ]
  },
  "dijkstra.rs": {
    "prefix": "dijkstra.rs",
    "body": [
      "mod dijkstra {",
      "    use std::cmp::Reverse;",
      "    use std::collections::BinaryHeap;",
      "",
      "    #[derive(Debug, Clone, Copy)]",
      "    pub struct Edge {",
      "        to: usize,",
      "        weight: usize,",
      "    }",
      "",
      "    impl Edge {",
      "        pub fn new(to: usize, weight: usize) -> Edge {",
      "            Edge { to, weight }",
      "        }",
      "    }",
      "",
      "    pub fn dijkstra(graph: Vec<Vec<Edge>>, start: usize) -> Vec<usize> {",
      "        const INF: usize = 1_000_000_000 + 7;",
      "        let size = graph.len();",
      "        let mut queue = BinaryHeap::new();",
      "        let mut dist = vec![INF; size];",
      "",
      "        queue.push(Reverse((0, start)));",
      "        dist[start] = 0;",
      "",
      "        while !queue.is_empty() {",
      "            let Reverse((d, v)) = queue.pop().unwrap();",
      "            if dist[v] < d {",
      "                continue;",
      "            }",
      "            for Edge { to, weight } in &graph[v] {",
      "                if dist[*to] > dist[v] + *weight {",
      "                    dist[*to] = dist[v] + *weight;",
      "                    queue.push(Reverse((dist[*to], *to)));",
      "                }",
      "            }",
      "        }",
      "",
      "        dist",
      "    }",
      "}"
    ]
  },
  
}
```

これをvscodeのユーザースニペットに登録することでライブラリ名を入力することで展開することができます。
