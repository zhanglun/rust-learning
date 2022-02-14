

  import { Channel as ChannelModel, Article as ArticleModel} from "../db";

  export const parseFeedXML = (xml: string): { channel: Omit<ChannelModel, 'feedUrl'>, items: Omit<ArticleModel, 'feedUrl' | 'unRead'>[] } => {
    const parser = new DOMParser();
    const dom = parser.parseFromString(xml, "application/xml");

    const parseChannel = (channeldom: any) => {
      const title = channeldom.querySelector("title").textContent;
      const link = channeldom.querySelector("link").textContent;
      const description = channeldom.querySelector("description").textContent;
      const pubDate = channeldom.querySelector("pubDate").textContent;

      return {
        title,
        link,
        description,
        pubDate,
      };
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

    let channel = {} as Omit<ChannelModel, 'feedUrl'>;
    let items = [] as Omit<ArticleModel, 'feedUrl' | 'unRead'>[];

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

  export const extendFeedItems = (items:  Omit<ArticleModel, 'feedUrl' | 'unRead'>[] , data: any) => {
    return items.map((item) => {
      return {
        ...items,
        ...data,
      }
    });
  }

  export const extendChannel = (channel: Omit<ChannelModel, 'feedUrl'>, data: any) => {
    return {
      ...channel,
      ...data,
    }
  }
