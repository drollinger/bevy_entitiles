# 导入来自LDtk的地图

如果你不知道什么是 [`LDtk`](https://ldtk.io) ，那么非常建议你去了解一下这款超棒的地图编辑器！当然如果你不使用LDtk，那么你可以跳过这一章。本章内容很多，请做好准备。

## [`ldtk.rs`](https://github.com/443eb9/bevy_entitiles/blob/0.4.0/examples/ldtk.rs)

点进这个示例，首先你会看到 `App` 后面跟着的一顿东西。

首先看到 [`68` 行](https://github.com/443eb9/bevy_entitiles/blob/0.4.0/examples/ldtk.rs#L68) 的 [`LdtkLoadConfig`](https://github.com/443eb9/bevy_entitiles/blob/0.4.0/src/ldtk/resources.rs#L304) 。

```rust
pub struct LdtkLoadConfig {
    pub file_path: String,
    pub asset_path_prefix: String,
    pub filter_mode: FilterMode,
    pub z_index: i32,
    pub animation_mapper: HashMap<u32, RawTileAnimation>,
    pub ignore_unregistered_entities: bool,
    pub ignore_unregistered_entity_tags: bool,
}
```

| 成员变量                          | 用途                                                                                                                                                                                                      |
| --------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `file_path`                       | `*.ldtk` 文件的路径，由于 `entitiles` 直接调用 `std::fs::read_to_string()` 来读取文件，你需要加上 `assets/` 前缀。                                                                                        |
| `asset_path_prefix`               | 然而图像等asset还是需要通过 `AssetServer` 读取的。如果你的地图在 `assets/ldtk/map.ldtk`，那么你需要设置这个值为 `ldtk` ，因为在 `*.ldtk` 中，资源文件是由相对路径表示的。所以需要加上这个前缀来正确读取。 |
| `filter_mode`                     | 同 `TextureDescriptor` 中的 `filter_mode`。                                                                                                                                                               |
| `z_index`                         | 基准`z`索引，导入的`ldtk`文件中的每一层都会由一张单独的地图展示，因此这个值决定了他们的最大的`z_index`，例如第一层为 `z` ，第二层为 `z - 1`。                                                             |
| `animation_mapper`                | 将文件中某些特定 `texture_index` 的Tile的材质映射为对应的动画。                                                                                                                                           |
| `ignore_unregistered_entities`    | 忽略没有被注册的来自Ldtk的实体。如果关闭的状态下遇到了未注册的实体则会触发panic                                                                                                                           |
| `ignore_unregistered_entity_tags` | 忽略没有被注册的Ldtk实体的Tags。你可以在Ldtk中的左上角 `Entities` 页面任意实体的 `Entity Settings` 中找到。                                                                                               |

之后是 [`85` 行](https://github.com/443eb9/bevy_entitiles/blob/0.4.0/examples/ldtk.rs#L85C26-L85C47) 的 [`LdtkAdditionalLayers`](https://github.com/443eb9/bevy_entitiles/blob/0.4.0/src/ldtk/resources.rs#L296)

```rust
pub struct LdtkAdditionalLayers {
    #[cfg(feature = "algorithm")]
    pub path_layer: Option<LdtkPathLayer>,
    #[cfg(feature = "physics")]
    pub physics_layer: Option<LdtkPhysicsLayer>,
}
```

可以看到这个Resource是依赖于 `algorithm` 和 `physics` 特性的。这两个实际上就是对Ldtk中的IntGrid做的映射，使之变为特殊的算法层/物理层。

对于 `physics_layer`，实际上就是变相的 `DataPhysicsTilemap`。其中的 `parent` 变量代表生成的 `PhysicsTilemap` 应该挂在哪一层对应的实体上。而 `identifier` 则是对应的来自LDtk的层的名字。

对于 `path_layer` 则更为易懂。这里就做介绍了。

接下来就是一长串的 `register_ldtk_entity::<T>()` 我猜你已经猜到了这是干什么用的了。这个函数就是添加将你在LDtk中对实体的名字转换为实际组件的工具。它接受的参数就是LDtk中的 `Entity Identifer` 你可以在 `Project Entities` 面板中找到。`register_ldtk_entity_tag::<T>()` 同理。

然后就是 [`136` 行](https://github.com/443eb9/bevy_entitiles/blob/0.4.0/examples/ldtk.rs#L136) [`LdtkLevelManager`](https://github.com/443eb9/bevy_entitiles/blob/0.4.0/src/ldtk/resources.rs#L317)。它是管理所有关卡的工具，你可以加载/卸载/切换以及重载文件。

在 [`hot_reload()`](https://github.com/443eb9/bevy_entitiles/blob/0.4.0/examples/ldtk.rs#L154) 中，你见到了 [`LdtkAssets`](https://github.com/443eb9/bevy_entitiles/blob/0.4.0/src/ldtk/resources.rs#L95)。它存储着所有与这个LDtk文件有关的资产，包括tileset，对应LDtk Entity的Mesh和材质等等。它也提供了公共方法，方便你拿取。你会在一会的回调函数的参数中再次看到这个。

在 [`events()`](https://github.com/443eb9/bevy_entitiles/blob/0.4.0/examples/ldtk.rs#L178) 中，展示了一些相关的事件。

接下来就是我刚才说的回调函数了，也就是 [`player_spawn`](https://github.com/443eb9/bevy_entitiles/blob/0.4.0/examples/ldtk.rs#L215) ，这个函数会在proc-marco内部被调用，可以让你很方便地添加一些自定义的组件，同时不需要你自己实现一整个 `LdtkEntity` trait。

### `LdtkEntity`

最后就是proc-marco的内容了，该部分很大受启发于 [`bevy_ecs_ldtk`](https://github.com/Trouv/bevy_ecs_ldtk)。

首先看 `LdtkEntity`。你可以先跳过那个 `LdtkEnum` 。相信你已经知道了，要注册该实体，必须先实现 `LdtkEntity` trait。在这个proc-macro上有很多attribute

| attribute       | 作用                                                                                                                                                                                                                                                                                                    |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ldtk_default`  | 标识LDtk文件内没有定义的变量，比如示例中的 `mp`                                                                                                                                                                                                                                                         |
| `ldtk_name`     | 重命名proc-macro中对于该字段的名字，比如LDtk中叫 `HP` 的就可以重命名成 `hp`                                                                                                                                                                                                                             |
| `spawn_sprite`  | 生成该实体的sprite。当然你也可以选择不生成，比如示例里的一些Area类型的就可以不生成。但是为了展示更多支持的 [`TileRenderMode`](https://github.com/443eb9/bevy_entitiles/blob/0.4.0/src/ldtk/sprite.rs#L55)，还是都显示了。这个 `TileRenderMode` 对应 `Entity Settings` 中的 `Editor Visual` 右侧的选项。 |
| `global_entity` | 表示这个实体不属于任何层，它只有一个并且不会随着所属层的销毁而销毁。                                                                                                                                                                                                                                    |
| `callback`      | 指定上文所说的回调函数。                                                                                                                                                                                                                                                                                |

**值得注意的是，如果你需要使用自定义的枚举类型，你必须使用对应的Wrapper。**

这一段是原因，不想看的可以跳过。Rust不允许实现非本crate的trait给非本crate的结构体。在这里我们需要实现 `Into<T> for FieldInstance` (`T` 表示 `Option<YourEnum> or Option<Vec<YourEnum>>`)，`FieldInstance` 就是存储在LDtk中定义的值的对象，在反序列化过程中需要调用 `<FieldInstance as Into<T>>::into()`，而此处不允许实现 `Into<T> for FieldInstance`。因此只能在这里定义一个Wrapper然后实现 `Into<Wrapper> for FieldInstance`.

### `LdtkEnum`

这个比较简单，而Wrapper出现的原因也已经讲明。一共就 `ldtk_name` 和 `wrapper_derive` 两个较为易懂的attribute。此处不做介绍了。

### `LdtkEntityTag`

也比较简单，就是标志了对应Tag的组件，别忘了 `register_ldtk_entity_tag::<T>()`！

## [`ldtk_wfc.rs`](https://github.com/443eb9/bevy_entitiles/blob/0.4.0/examples/ldtk_wfc.rs)

这个示例介绍了如何使用LDtk的地图进行wfc。

这里只讲述区别于一般wfc的步骤。如果你需要一般wfc的教程，请看第3章。

其实也就唯一一行，定义 `WfcSource` 时选择了 `WfcSource::LdtkMapPattern()` 。这使得 `LdtkLevelManager` 会自动加载所有的关卡并作为wfc的选项。其中你可以选择将结果应用到一张地图上，也可以选择应用到多张地图上。

剩余的代码只在你选择 `LdtkWfcMode::MultiMaps` 才会有效，简单来说就是检测小方块移动到了哪个关卡，并且使 `LdtkLevelManager` 加载对应关卡。

<hr>

# Importing Maps from LDtk

If you're not familiar with [`LDtk`](https://ldtk.io), I highly recommend checking out this fantastic map editor! However, if you don't use LDtk, you can skip this chapter. There's a lot of content in this chapter, so be prepared.

## [`ldtk.rs`](https://github.com/443eb9/bevy_entitiles/blob/0.4.0/examples/ldtk.rs)

When you enter this example, you'll first see a bunch of stuff after `App`.

First, take a look at [`LdtkLoadConfig`](https://github.com/443eb9/bevy_entitiles/blob/0.4.0/src/ldtk/resources.rs#L304) at [line 68](https://github.com/443eb9/bevy_entitiles/blob/0.4.0/examples/ldtk.rs#L68).

```rust
pub struct LdtkLoadConfig {
    pub file_path: String,
    pub asset_path_prefix: String,
    pub filter_mode: FilterMode,
    pub z_index: i32,
    pub animation_mapper: HashMap<u32, RawTileAnimation>,
    pub ignore_unregistered_entities: bool,
    pub ignore_unregistered_entity_tags: bool,
}
```

| Member Variable                  | Purpose                                                                                                                                                                                               |
| --------------------------------| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `file_path`                      | Path to the `*.ldtk` file. Since `entitiles` directly uses `std::fs::read_to_string()` to read files, you need to add the `assets/` prefix.                                                          |
| `asset_path_prefix`              | However, images and other assets still need to be read through `AssetServer`. If your map is in `assets/ldtk/map.ldtk`, you need to set this value to `ldtk`, because resources in `*.ldtk` are represented by relative paths. So you need to add this prefix to read them correctly. |
| `filter_mode`                    | Same as `filter_mode` in `TextureDescriptor`.                                                                                                                                                          |
| `z_index`                        | Baseline `z` index. Each layer imported from the `ldtk` file will be displayed as a separate map, so this value determines their maximum `z_index`, where the first layer is `z` and the second layer is `z - 1`, and so on.                       |
| `animation_mapper`               | Maps the texture of certain specific `texture_index` Tiles in the file to corresponding animations.                                                                                                  |
| `ignore_unregistered_entities`   | Ignore unregistered entities from LDtk. If closed, encountering an unregistered entity will trigger a panic.                                                                                          |
| `ignore_unregistered_entity_tags`| Ignore unregistered LDtk entity tags. You can find them in the `Entity Settings` of any entity on the `Entities` page in LDtk's top-left corner.                                                      |

Next is [`LdtkAdditionalLayers`](https://github.com/443eb9/bevy_entitiles/blob/0.4.0/src/ldtk/resources.rs#L296) at [line 85](https://github.com/443eb9/bevy_entitiles/blob/0.4.0/examples/ldtk.rs#L85C26-L85C47).

```rust
pub struct LdtkAdditionalLayers {
    #[cfg(feature = "algorithm")]
    pub path_layer: Option<LdtkPathLayer>,
    #[cfg(feature = "physics")]
    pub physics_layer: Option<LdtkPhysicsLayer>,
}
```

This resource depends on the `algorithm` and `physics` features. These two essentially map to IntGrids in LDtk, making them special algorithm layers/physics layers.

For `physics_layer`, it's actually a disguised `DataPhysicsTilemap`. The `parent` variable indicates which layer the generated `PhysicsTilemap` should be attached to, while `identifier` is the name of the corresponding LDtk layer.

`path_layer` is easier to understand. It's explained here.

Next is a long list of `register_ldtk_entity::<T>()`. I guess you already know what this is for. This function adds a tool to translate the names of entities in LDtk into actual components for you. It accepts the `Entity Identifier` from LDtk, which you can find in the `Project Entities` panel. `register_ldtk_entity_tag::<T>()` works similarly.

Then there's [`LdtkLevelManager`](https://github.com/443eb9/bevy_entitiles/blob/0.4.0/src/ldtk/resources.rs#L317) at [line 136](https://github.com/443eb9/bevy_entitiles/blob/0.4.0/examples/ldtk.rs#L136). It's a tool for managing all levels, allowing you to load/unload/switch and reload files.

In [`hot_reload()`](https://github.com/443eb9/bevy_entitiles/blob/0.4.0/examples/ldtk.rs#L154), you'll see [`LdtkAssets`](https://github.com/443eb9/bevy_entitiles/blob/0.4.0/src/ldtk/resources.rs#L95). It stores all assets related to this LDtk file, including tilesets, meshes, and materials for LDtk entities. It also provides common methods for you to retrieve them. You'll see this again in the parameters of the callback function shortly.

In `events()`, some related events are showcased.

Next is the callback function I mentioned earlier, namely `player_spawn`, which is called internally within the proc-macro. It allows you to conveniently add some custom components without implementing the entire `LdtkEntity` trait yourself.

### `LdtkEntity`

Finally, let's talk about the proc-macro part, which is heavily inspired by [`bevy_ecs_ldtk`](https://github.com/Trouv/bevy_ecs_ldtk).

Let's start with `LdtkEntity`. You can skip `LdtkEnum` for now. As you might have guessed, to register an entity, you must first implement the `LdtkEntity` trait. There are many attributes on this proc-macro.

| Attribute        | Purpose                                                                                                                                                                                                                                         |
| ---------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ldtk_default`   | Indicates variables not defined in the LDtk file, like `mp` in the example.                                                                                                                                                                    |
| `ldtk_name`      | Renames the name of the field in the proc-macro; for example, what's called `HP` in LDtk can be renamed to `hp`.                                                                                                                               |
| `spawn_sprite`   | Generates a sprite for this entity. You can choose not to generate, like some Area types in the example. However, to demonstrate more supported [`TileRenderMode`](https://github.com/443eb9/bevy_entitiles/blob/0.4.0/src/ldtk/sprite.rs#L55), all are displayed. This corresponds to the options on the right side of `Editor Visual` in `Entity Settings`. |
| `global_entity`  | Indicates that this entity doesn't belong to any layer; it exists independently and won't be destroyed with its layer.                                                                                                                                                                                      |
| `callback`       | Specifies the callback function mentioned earlier.                                                                                                                                                                                              |

**Note: If you need to use custom enum types, you must use the corresponding wrapper.**

The reason for this is that Rust doesn't allow implementing a trait from another crate for a struct from a different crate. Here we need to implement `Into<T> for FieldInstance` (`T` represents `Option<YourEnum> or Option<Vec<YourEnum>>`), where `FieldInstance` is the object that stores the values defined in LDtk, and during deserialization, it needs to call `<FieldInstance as Into<T>>::into()`. However, it's not allowed to implement `Into<T> for FieldInstance` here. So we define a wrapper here and implement `Into<Wrapper> for FieldInstance`.

### `LdtkEnum`

This is relatively simple, and the reason for the Wrapper has been explained. There are only two understandable attributes: `ldtk_name` and `wrapper_derive`.

### `LdtkEntityTag`

It's also straightforward, marking the component for the corresponding Tag. Don't forget to `register_ldtk_entity_tag::<T>()`!

## [`ldtk_wfc.rs`](https://github.com/443eb9/bevy_entitiles/blob/0.4.0/examples/ldtk_wfc.rs)

This example shows how to use LDtk maps for wfc.

This only describes the steps different from regular wfc. If you need a tutorial on regular wfc, please refer to chapter 3.

Actually, there's only one line; when defining `WfcSource`, choose `WfcSource::LdtkMapPattern()`. This makes `LdtkLevelManager` automatically load all levels as options for wfc. Here, you can choose to apply the results to one map or multiple maps.

The remaining code only works if you choose `LdtkWfcMode::MultiMaps`, which essentially checks which level the small square has moved to and makes `LdtkLevelManager` load the corresponding level.
