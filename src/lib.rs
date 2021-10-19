use std::convert::TryInto;
use std::time::Duration;
use surf::{Client, Config, Url};

pub mod captions;
pub mod channel_banners;
pub mod channel_sections;
pub mod channels;
pub mod comment_threads;
pub mod comments;
pub mod i18n_languages;
pub mod i18n_regions;
pub mod members;
pub mod memberships_levels;
pub mod playlist_items;
pub mod playlists;
pub mod search;
pub mod subscriptions;
pub mod video_abuse_report_reasons;
pub mod video_categories;
pub mod videos;
pub mod watermarks;

pub struct Youtube {
  pub captions: captions::CaptionsService,
  pub channel_banners: channel_banners::ChannelBannersService,
  pub channel_sections: channel_sections::ChannelSectionsService,
  pub channels: channels::ChannelsService,
  pub comment_threads: comment_threads::CommentThreadsService,
  pub comments: comments::CommentsService,
  pub i18n_languages: i18n_languages::I18nLanguagesService,
  pub i18n_regions: i18n_regions::I18nRegionsService,
  pub members: members::MembersService,
  pub memberships_levels: memberships_levels::MembershipsLevelsService,
  pub playlist_items: playlist_items::PlaylistItemsService,
  pub playlists: playlists::PlaylistsService,
  pub search: search::SearchService,
  pub subscriptions: subscriptions::SubscriptionsService,
  pub video_abuse_report_reasons: video_abuse_report_reasons::VideoAbuseReportReasonsService,
  pub video_categories: video_categories::VideoCategoriesService,
  pub videos: videos::VideosService,
  pub watermarks: watermarks::WatermarksService,
}

const BASE_URL: &str = "https://www.googleapis.com/youtube/v3/";

impl Youtube {
  pub fn new() -> Self {
    let client: Client = Config::new()
      .set_base_url(Url::parse(BASE_URL).unwrap())
      .set_timeout(Some(Duration::from_secs(5)))
      .try_into()
      .unwrap();
    Self {
      captions: captions::CaptionsService::new(&client),
      channel_banners: channel_banners::ChannelBannersService::new(&client),
      channel_sections: channel_sections::ChannelSectionsService::new(&client),
      channels: channels::ChannelsService::new(&client),
      comment_threads: comment_threads::CommentThreadsService::new(&client),
      comments: comments::CommentsService::new(&client),
      i18n_languages: i18n_languages::I18nLanguagesService::new(&client),
      i18n_regions: i18n_regions::I18nRegionsService::new(&client),
      members: members::MembersService::new(&client),
      memberships_levels: memberships_levels::MembershipsLevelsService::new(&client),
      playlist_items: playlist_items::PlaylistItemsService::new(&client),
      playlists: playlists::PlaylistsService::new(&client),
      search: search::SearchService::new(&client),
      subscriptions: subscriptions::SubscriptionsService::new(&client),
      video_abuse_report_reasons: video_abuse_report_reasons::VideoAbuseReportReasonsService::new(
        &client,
      ),
      video_categories: video_categories::VideoCategoriesService::new(&client),
      videos: videos::VideosService::new(&client),
      watermarks: watermarks::WatermarksService::new(&client),
    }
  }
}
