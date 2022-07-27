pub struct RatingQuery {
    spot_id: TaxonomyID,
    ndays: i8,
    interval: i8,
    corrected_wind: Option<bool>
}