pub struct WaveQuery {
    spot_id: TaxonomyID,
    ndays: i8,
    interval: i8,
}

pub struct WaveRes {
    meta: WaveMetadata,
    points: [WavePoint],
}

// Might want to reconsider if we want so much nesting
pub struct WavePoint {
    timestamp: u64,
    utc_offset: i8,
    surf: SurfPoint,
    swells: [SwellPoint],
}

// Really need to reconsider these names
pub struct SurfPoint {
    min: f32,
    max: f32,
    optimal_score: Option<i32>,
    plus: bool,
    human_relation: HumanRelation,
    raw: RawSurf,
}

pub struct RawSurf {
    min: f32,
    max: f32,
}

pub struct SwellPoint {
    height: f32,
    period: i32,
    direction: f32,
    min_direction: f32,
    optimal_score: i32,
}
