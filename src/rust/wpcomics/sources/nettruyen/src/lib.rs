#![no_std]
pub mod helper;
use crate::helper::*;
use aidoku::{
	error::Result,
	prelude::*,
	std::net::Request,
	std::String,
	std::{defaults::defaults_get, Vec},
	Chapter, DeepLink, Filter, FilterType, Listing, Manga, MangaPageResult, MangaViewer, Page,
};
use wpcomics_template::{helper::urlencode, template, template::Selectors};

static SELECTORS: Selectors = Selectors {
	next_page: "li.active + li > a[title*=\"kết quả\"]",
	manga_cell: "div.items > div.row > div.item > figure.clearfix",
	manga_cell_title: "figcaption > h3 > a",
	manga_cell_url: "div.image > a",
	manga_cell_image: "div.image > a > img",

	manga_details_title: "h1.title-detail",
	manga_details_title_transformer: |title| title,
	manga_details_cover: "div.col-image > img",
	manga_details_author: "ul.list-info > li.author > p.col-xs-8",
	manga_details_author_transformer: |title| title,
	manga_details_description: "div.detail-content > p",
	manga_details_tags: "li.kind.row > p.col-xs-8",
	manga_details_tags_splitter: " - ",
	manga_details_status: "li.status.row > p.col-xs-8",
	manga_details_status_transformer: |title| title,
	manga_details_chapters: "div.list-chapter > nav > ul > li",

	manga_viewer_page: "div.page-chapter > img",

	chapter_anchor_selector: "div.chapter > a",
	chapter_date_selector: "div.col-xs-4",
};

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
	let mut included_tags: Vec<String> = Vec::new();
	let mut excluded_tags: Vec<String> = Vec::new();
	let mut title: String = String::new();
	let mut sort_by: i32 = 0;
	let mut gender: i32 = -1;
	let mut completed: i32 = -1;
	let mut chapter_count: i32 = 0;
	for filter in filters {
		match filter.kind {
			FilterType::Title => {
				title = urlencode(filter.value.as_string()?.read());
			}
			FilterType::Genre => match filter.value.as_int().unwrap_or(-1) {
				0 => excluded_tags.push(get_tag_id(String::from(&filter.name))),
				1 => included_tags.push(get_tag_id(String::from(&filter.name))),
				_ => continue,
			},
			_ => {
				match filter.name.as_str() {
					"Tình trạng" => {
						completed = filter.value.as_int().unwrap_or(-1) as i32;
						if completed == 0 {
							completed = -1;
						}
					}
					"Số lượng chapter" => {
						chapter_count = match filter.value.as_int().unwrap_or(0) {
							0 => 1,
							1 => 50,
							2 => 100,
							3 => 200,
							4 => 300,
							5 => 400,
							6 => 500,
							_ => 1,
						};
					}
					"Sắp xếp theo" => {
						sort_by = match filter.value.as_int().unwrap_or(0) {
							0 => 0,	 // new chapters
							1 => 15, // new mangas
							2 => 10, // most watched
							3 => 11, // most watched this month
							4 => 12, // most watched this week
							5 => 13, // most watched today
							6 => 20, // most followed
							7 => 25, // most commented
							8 => 30, // most chapters
							9 => 5,	 // alphabetical
							_ => 0,
						};
					}
					"Dành cho" => {
						gender = filter.value.as_int().unwrap_or(-1) as i32;
						if gender == 0 {
							gender = -1;
						}
					}
					_ => continue,
				}
			}
		}
	}
	template::get_manga_list(
		get_search_url(
			String::from("https://www.nettruyenco.com"),
			title,
			page,
			included_tags,
			excluded_tags,
			sort_by,
			gender,
			completed,
			chapter_count,
		),
		&SELECTORS,
	)
}

#[get_manga_listing]
fn get_manga_listing(listing: Listing, page: i32) -> Result<MangaPageResult> {
	template::get_manga_listing(
		String::from("https://www.nettruyenco.com"),
		listing,
		&SELECTORS,
		listing_map,
		page,
	)
}

#[get_manga_details]
fn get_manga_details(id: String) -> Result<Manga> {
	template::get_manga_details(id, &SELECTORS, MangaViewer::Rtl, status_map)
}

#[get_chapter_list]
fn get_chapter_list(id: String) -> Result<Vec<Chapter>> {
	template::get_chapter_list(id, &SELECTORS, false, convert_time)
}

#[get_page_list]
fn get_page_list(id: String) -> Result<Vec<Page>> {
	template::get_page_list(id, &SELECTORS, String::from(""), |url| {
		let mut server_two = String::from("https://images2-focus-opensocial.googleusercontent.com/gadgets/proxy?container=focus&gadget=a&no_expand=1&resize_h=0&rewriteMime=image%2F*&url=");
		if let Ok(server_selection) = defaults_get("serverSelection").as_int() {
			match server_selection {
				2 => {
					server_two.push_str(&urlencode(url));
					return server_two;
				}
				_ => {
					return url;
				}
			}
		} else {
			return url;
		}
	})
}

#[modify_image_request]
fn modify_image_request(request: Request) {
	template::modify_image_request(
		String::from("https://www.nettruyenco.com"),
		String::from("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.54 Safari/537.36 Edg/101.0.1210.39"),
		request,
	)
}

#[handle_url]
pub fn handle_url(url: String) -> Result<DeepLink> {
	template::handle_url(url, &SELECTORS, MangaViewer::Rtl, status_map)
}