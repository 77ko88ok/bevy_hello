use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            meta_check: bevy::asset::AssetMetaCheck::Never, // これが必要
            ..default()
            )
        .add_systems(Startup, setup) // セットアップシステムを追加
        .run();
}

// セットアップ関数.
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // カメラの追加 (描画に必要).
    commands.spawn(Camera2d);

    // 画像の読み込み.
    let image = asset_server.load("images/papa.png");

    // 画像を表示するスプライトを作成.
    commands.spawn((
        Sprite::from_image(image),
        Transform::from_xyz(0.0, 0.0, 0.0), // 位置
    ));
}
