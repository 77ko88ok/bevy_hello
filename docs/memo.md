### coding tips
#### wasmでアセットのメタデータを読みこまないエラーについては、asset::AssetMetaCheck::Neverにすると回避できる
~~~rust
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                }),
        ))
        .add_systems(Startup, setup)
        .run();
}
~~~

#### github actionsでgithub pagesにデプロイするときには、assetを明示的にdistにコピーする必要がある
~~~yaml
      # 4. リリースモードでWASMビルドを実行
      - name: Build with Trunk
        run: trunk build --release --public-url "/bevy_hello/"

      - name: Copy assets to dist
        run: cp -r assets dist
~~~