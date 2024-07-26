use bevy::{
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
    utils::HashMap,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<HandleMap<ImageKey>>();
    app.init_resource::<HandleMap<ImageKey>>();

    app.register_type::<HandleMap<SfxKey>>();
    app.init_resource::<HandleMap<SfxKey>>();

    app.register_type::<HandleMap<SoundtrackKey>>();
    app.init_resource::<HandleMap<SoundtrackKey>>();
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum ImageKey {
    Ducky,
    Table,       //2048x1024
    Tockery,     //409x930
    Clock,       //256
    ClockHour,   //128
    ClockMinute, //128
    MainClock,
    MainClockHour,
    MainClockMinute,
    OilFull,
    Oil90,
    Oil80,
    Oil70,
    Oil60,
    Oil50,
    Oil40,
    Oil30,
    Oil20,
    Oil10,
    OilEmpty,
    OilCan,
    ClockTable,
    OilTable,
    Background,
}

impl AssetKey for ImageKey {
    type Asset = Image;
}

impl FromWorld for HandleMap<ImageKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [
            (
                ImageKey::Ducky,
                asset_server.load_with_settings(
                    "images/ducky.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest();
                    },
                ),
            ),
            (
                ImageKey::Tockery,
                asset_server.load_with_settings(
                    "images/new-tickery-tockery.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest();
                    },
                ),
            ),
            (
                ImageKey::Table,
                asset_server.load_with_settings(
                    "images/table.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest();
                    },
                ),
            ),
            (
                ImageKey::Clock,
                asset_server.load_with_settings(
                    "images/new-clock.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest();
                    },
                ),
            ),
            (
                ImageKey::ClockHour,
                asset_server.load_with_settings(
                    "images/new-clock-hour.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest();
                    },
                ),
            ),
            (
                ImageKey::ClockMinute,
                asset_server.load_with_settings(
                    "images/new-clock-minute.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest();
                    },
                ),
            ),
            (
                ImageKey::MainClock,
                asset_server.load_with_settings(
                    "images/main-clock.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest();
                    },
                ),
            ),
            (
                ImageKey::MainClockHour,
                asset_server.load_with_settings(
                    "images/main-clock-hour.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest();
                    },
                ),
            ),
            (
                ImageKey::MainClockMinute,
                asset_server.load_with_settings(
                    "images/main-clock-minute.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest();
                    },
                ),
            ),
            (
                ImageKey::OilFull,
                asset_server.load_with_settings(
                    "images/oil-full.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest();
                    },
                ),
            ),
            (
                ImageKey::Oil90,
                asset_server.load_with_settings(
                    "images/oil-90.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest();
                    },
                ),
            ),
            (
                ImageKey::Oil80,
                asset_server.load_with_settings(
                    "images/oil-80.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest();
                    },
                ),
            ),
            (
                ImageKey::Oil70,
                asset_server.load_with_settings(
                    "images/oil-70.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest();
                    },
                ),
            ),
            (
                ImageKey::Oil60,
                asset_server.load_with_settings(
                    "images/oil-60.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest();
                    },
                ),
            ),
            (
                ImageKey::Oil50,
                asset_server.load_with_settings(
                    "images/oil-50.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest();
                    },
                ),
            ),
            (
                ImageKey::Oil40,
                asset_server.load_with_settings(
                    "images/oil-40.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest();
                    },
                ),
            ),
            (
                ImageKey::Oil30,
                asset_server.load_with_settings(
                    "images/oil-30.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest();
                    },
                ),
            ),
            (
                ImageKey::Oil20,
                asset_server.load_with_settings(
                    "images/oil-20.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest();
                    },
                ),
            ),
            (
                ImageKey::Oil10,
                asset_server.load_with_settings(
                    "images/oil-10.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest();
                    },
                ),
            ),
            (
                ImageKey::OilCan,
                asset_server.load_with_settings(
                    "images/oil.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest();
                    },
                ),
            ),
            (
                ImageKey::ClockTable,
                asset_server.load_with_settings(
                    "images/clock-table.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest();
                    },
                ),
            ),
            (
                ImageKey::OilTable,
                asset_server.load_with_settings(
                    "images/oil-table.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest();
                    },
                ),
            ),
            (
                ImageKey::Background,
                asset_server.load_with_settings(
                    "images/background.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest();
                    },
                ),
            ),
        ]
        .into()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum SfxKey {
    ButtonHover,
    ButtonPress,
    Step1,
    Step2,
    Step3,
    Step4,
    OilDrink,
}

impl AssetKey for SfxKey {
    type Asset = AudioSource;
}

impl FromWorld for HandleMap<SfxKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [
            (
                SfxKey::ButtonHover,
                asset_server.load("audio/sfx/button_hover.ogg"),
            ),
            (
                SfxKey::ButtonPress,
                asset_server.load("audio/sfx/button_press.ogg"),
            ),
            (SfxKey::Step1, asset_server.load("audio/sfx/step1.ogg")),
            (SfxKey::Step2, asset_server.load("audio/sfx/step2.ogg")),
            (SfxKey::Step3, asset_server.load("audio/sfx/step3.ogg")),
            (SfxKey::Step4, asset_server.load("audio/sfx/step4.ogg")),
            (SfxKey::OilDrink, asset_server.load("audio/sfx/oil.wav")),
        ]
        .into()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum SoundtrackKey {
    Credits,
    Gameplay,
}

impl AssetKey for SoundtrackKey {
    type Asset = AudioSource;
}

impl FromWorld for HandleMap<SoundtrackKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [
            (
                SoundtrackKey::Credits,
                asset_server.load("audio/soundtracks/Monkeys Spinning Monkeys.ogg"),
            ),
            (
                SoundtrackKey::Gameplay,
                asset_server.load("audio/soundtracks/Fluffing A Duck.ogg"),
            ),
        ]
        .into()
    }
}

pub trait AssetKey: Sized {
    type Asset: Asset;
}

#[derive(Resource, Reflect, Deref, DerefMut)]
#[reflect(Resource)]
pub struct HandleMap<K: AssetKey>(HashMap<K, Handle<K::Asset>>);

impl<K: AssetKey, T> From<T> for HandleMap<K>
where
    T: Into<HashMap<K, Handle<K::Asset>>>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl<K: AssetKey> HandleMap<K> {
    pub fn all_loaded(&self, asset_server: &AssetServer) -> bool {
        self.values()
            .all(|x| asset_server.is_loaded_with_dependencies(x))
    }
}
