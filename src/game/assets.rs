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

    app.register_type::<HandleMap<FontKey>>();
    app.init_resource::<HandleMap<FontKey>>();
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum FontKey {
    Arrancar,
    Guavine,
}

impl AssetKey for FontKey {
    type Asset = Font;
}

impl FromWorld for HandleMap<FontKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [
            (FontKey::Arrancar, asset_server.load("fonts/arrancar.ttf")),
            (FontKey::Guavine, asset_server.load("fonts/guavine.otf")),
        ]
        .into()
    }
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
    TitleBackground,
    Gear,
    TitleHand,
    StartButton,
    CreditsButton,
    SubmitButton,
    LeaderboardButton,
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
            (
                ImageKey::TitleBackground,
                asset_server.load_with_settings(
                    "images/title-background.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest();
                    },
                ),
            ),
            (
                ImageKey::Gear,
                asset_server.load_with_settings(
                    "images/gear.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest();
                    },
                ),
            ),
            (
                ImageKey::TitleHand,
                asset_server.load_with_settings(
                    "images/title-hand.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest();
                    },
                ),
            ),
            (
                ImageKey::StartButton,
                asset_server.load_with_settings(
                    "images/start-button.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest();
                    },
                ),
            ),
            (
                ImageKey::CreditsButton,
                asset_server.load_with_settings(
                    "images/credits-button.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest();
                    },
                ),
            ),
            (
                ImageKey::SubmitButton,
                asset_server.load_with_settings(
                    "images/submit-button.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest();
                    },
                ),
            ),
            (
                ImageKey::LeaderboardButton,
                asset_server.load_with_settings(
                    "images/leaderboard-button.png",
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
    MenuTick,
    MenuTock,
    Ticking1,
    Ticking2,
    Ticking3,
    Ticking4,
    Ticking5,
    Ticking6,
    Setting1,
    Setting2,
    Setting3,
    Setting4,
    Setting5,
    Setting6,
    ClockDown1,
    ClockDown2,
    ClockDown3,
    ClockDown4,
    ClockSpawn1,
    ClockSpawn2,
    ClockSpawn3,
    ClockSpawn4,
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
            (
                SfxKey::MenuTick,
                asset_server.load("audio/sfx/menu-tick.wav"),
            ),
            (
                SfxKey::MenuTock,
                asset_server.load("audio/sfx/menu-tock.wav"),
            ),
            (
                SfxKey::Ticking1,
                asset_server.load("audio/sfx/ticking-1.wav"),
            ),
            (
                SfxKey::Ticking2,
                asset_server.load("audio/sfx/ticking-2.wav"),
            ),
            (
                SfxKey::Ticking3,
                asset_server.load("audio/sfx/ticking-3.wav"),
            ),
            (
                SfxKey::Ticking4,
                asset_server.load("audio/sfx/ticking-4.wav"),
            ),
            (
                SfxKey::Ticking5,
                asset_server.load("audio/sfx/ticking-5.wav"),
            ),
            (
                SfxKey::Ticking6,
                asset_server.load("audio/sfx/ticking-6.wav"),
            ),
            (SfxKey::Setting1, asset_server.load("audio/sfx/set-1.wav")),
            (SfxKey::Setting2, asset_server.load("audio/sfx/set-2.wav")),
            (SfxKey::Setting3, asset_server.load("audio/sfx/set-3.wav")),
            (SfxKey::Setting4, asset_server.load("audio/sfx/set-4.wav")),
            (SfxKey::Setting5, asset_server.load("audio/sfx/set-5.wav")),
            (SfxKey::Setting6, asset_server.load("audio/sfx/set-6.wav")),
            (
                SfxKey::ClockDown1,
                asset_server.load("audio/sfx/clock-down-1.wav"),
            ),
            (
                SfxKey::ClockDown2,
                asset_server.load("audio/sfx/clock-down-2.wav"),
            ),
            (
                SfxKey::ClockDown3,
                asset_server.load("audio/sfx/clock-down-3.wav"),
            ),
            (
                SfxKey::ClockDown4,
                asset_server.load("audio/sfx/clock-down-4.wav"),
            ),
            (
                SfxKey::ClockSpawn1,
                asset_server.load("audio/sfx/new-clock-1.wav"),
            ),
            (
                SfxKey::ClockSpawn2,
                asset_server.load("audio/sfx/new-clock-2.wav"),
            ),
            (
                SfxKey::ClockSpawn3,
                asset_server.load("audio/sfx/new-clock-3.wav"),
            ),
            (
                SfxKey::ClockSpawn4,
                asset_server.load("audio/sfx/new-clock-4.wav"),
            ),
        ]
        .into()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum SoundtrackKey {
    Credits,
    Gameplay,
    Menu,
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
                asset_server.load("audio/soundtracks/credits.wav"),
            ),
            (
                SoundtrackKey::Gameplay,
                asset_server.load("audio/soundtracks/game.wav"),
            ),
            (
                SoundtrackKey::Menu,
                asset_server.load("audio/soundtracks/menu.wav"),
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
