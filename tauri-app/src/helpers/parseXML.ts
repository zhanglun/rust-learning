import { http } from "@tauri-apps/api";
import { Channel as ChannelModel, Article as ArticleModel } from "../db";

type ChannelRes = Omit<ChannelModel, "feedUrl">;
type ArticelRes = Omit<ArticleModel, "feedUrl" | "unRead">;

export const parseFeedXML = (
  xml: string
): {
  channel: ChannelRes;
  items: ArticelRes[];
} => {
  const parser = new DOMParser();
  const dom = parser.parseFromString(xml, "application/xml");

  const parseChannel = (channeldom: any) => {
    const res = {} as Omit<ChannelModel, "feedUrl">;
    let child = channeldom.firstChild;

    while (true) {
      if (!child) {
        break;
      }

      switch (child.nodeName) {
        case "title":
          res.title = child.textContent;
          break;
        case "link":
          res.link = child.textContent;
          break;
        case "description":
          res.description = child.textContent;
          break;
        case "lastBuildDate":
        case "pubDate":
          res.pubDate = child.textContent;
          break;
        default:
          break;
      }

      child = child.nextElementSibling;
    }

    console.log("---->", channeldom);
    console.log("---->", channeldom.querySelector("link"));
    console.log(res);

    return res;
  };

  const parseItems = (doc: any) => {
    const items = doc.querySelectorAll("item");
    const res = [];

    for (let item of items) {
      const feed: any = {};
      let child = item.firstChild;

      while (true) {
        if (!child) {
          break;
        }

        const content = child.textContent;

        switch (child.nodeName) {
          case "title":
            feed.title = content;
            break;
          case "link":
            feed.link = content;
            break;
          case "description":
            feed.description = content;
            break;
          case "content":
          case "content:encoded":
            feed.content = content;
            break;
          case "author":
          case "dc:creator":
            feed.author = content;
            break;
          case "pubDate":
            feed.pubDate = content;
            break;
          default:
            break;
        }

        child = child.nextElementSibling;
      }

      console.log(feed);

      res.push(feed);
    }

    return res;
  };

  let channel = {} as Omit<ChannelModel, "feedUrl">;
  let items = [] as Omit<ArticleModel, "feedUrl" | "unRead">[];

  if (dom.querySelector("channel")) {
    channel = {
      ...parseChannel(dom.querySelector("channel")),
    };
  }

  if (dom.querySelector("item")) {
    items = parseItems(dom);
  }

  console.log(dom);

  return {
    channel,
    items,
  };
};

export const extendFeedItems = (
  items: Omit<ArticleModel, "feedUrl" | "unRead">[],
  data: any
) => {
  return items.map((item) => {
    return {
      ...item,
      ...data,
    };
  });
};

export const extendChannel = (
  channel: Omit<ChannelModel, "feedUrl">,
  data: any
) => {
  return {
    ...channel,
    ...data,
  };
};

export const requestFeed = (
  url: string
): Promise<{ channel: ChannelModel; items: ArticleModel[] } | any> => {
  return http
    .fetch(url, {
      method: "GET",
      responseType: 2,
    })
    .then(({ status, data }: any) => {
      if (status === 200) {
        const { channel, items } = parseFeedXML(data);

        return {
          channel: extendChannel(channel, { feedUrl: url }),
          items: extendFeedItems(items, { feedUrl: url, unRead: 0 }),
        };
      } else {
        console.log("-=--> http request Error", status, data);
        return {
          status,
          data,
        }
      }
    });
};

export const getFavico = (url: string) => {
  const hostname = url ? new URL(url).hostname : "";

  return "https://icons.duckduckgo.com/ip3/" + hostname + ".ico";
};
