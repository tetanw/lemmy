use crate::db::establish_connection;
use crate::db::site_view::SiteView;
use crate::version;
use crate::Settings;
use actix_web::body::Body;
use actix_web::HttpResponse;
use serde_json::json;

pub fn node_info_well_known() -> HttpResponse<Body> {
  let json = json!({
    "links": {
      "rel": "http://nodeinfo.diaspora.software/ns/schema/2.0",
      "href": format!("https://{}/nodeinfo/2.0.json", Settings::get().hostname),
    }
  });

  return HttpResponse::Ok()
    .content_type("application/json")
    .body(json.to_string());
}

pub fn node_info() -> HttpResponse<Body> {
  let conn = establish_connection();
  let site_view = match SiteView::read(&conn) {
    Ok(site_view) => site_view,
    Err(_e) => return HttpResponse::InternalServerError().finish(),
  };
  let json = json!({
    "version": "2.0",
    "software": {
      "name": "lemmy",
      "version": version::VERSION,
    },
    "protocols": [],
    "usage": {
      "users": {
        "total": site_view.number_of_users
      },
      "localPosts": site_view.number_of_posts,
      "localComments": site_view.number_of_comments,
      "openRegistrations": true,
      }
  });
  return HttpResponse::Ok()
    .content_type("application/json")
    .body(json.to_string());
}
