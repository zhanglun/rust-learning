import React, { useImperativeHandle, useState } from "react";
import { http } from "@tauri-apps/api";
import { Modal } from "../Modal";
import { useModal } from "../Modal/useModal";
import { db, Channel as ChannelModel, Article as ArticleModel } from "../../db";

export const AddFeedChannel = (props: any) => {
  const { showStatus, showModal, hideModal, toggleModal } = useModal();
  const [feedUrl, setFeedUrl] = useState("https://post.smzdm.com/feed");
  const [title, setTitle] = useState("");
  const [channel, setChannel] = useState({} as ChannelModel);
  const [articles, setArticles] = useState([] as ArticleModel[]);

  const parseFeedXML = (xml: string) => {
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

        res.push(feed);
      }

      return res;
    };

    let channel = {} as ChannelModel;
    let items = [] as ArticleModel[];

    if (dom.querySelector("channel")) {
      channel = {
        ...parseChannel(dom.querySelector("channel")),
        feedUrl,
      };
    }

    if (dom.querySelector("item")) {
      items = parseItems(dom);
      items.forEach((item) => {
        item.feedUrl = feedUrl;
      })
    }

    console.log(dom);

    return {
      channel,
      items,
    };
  };

  useImperativeHandle(props.Aref, () => {
    return {
      status: showStatus,
      showModal,
      hideModal,
      toggleModal,
    };
  });

  const handleLoad = () => {
    http
      .fetch(feedUrl, {
        method: "GET",
        responseType: 2,
      })
      .then(({ status, data }: any) => {
        if (status === 200) {
          const { channel, items } = parseFeedXML(data);

          setTitle(channel.title);
          setChannel(channel);
          setArticles(items);
        }
      });
  };

  const handleTitleChange = (e: any) => {
    setTitle(e.target.value);
  };

  const handleInputChange = (e: any) => {
    setFeedUrl(e.target.value);
  };

  const handleCancel = () => {
    toggleModal();
  };

  const handleSave = () => {
    db.channels.add(channel);
    db.articles.bulkAdd(articles);
  };

  return (
    <Modal visible={showStatus} toggle={toggleModal} title="添加 RSS 订阅">
      <div>
        <div>
          <div>Feed URL</div>
          <input type="text" value={feedUrl} onChange={handleInputChange} />
          <button onClick={handleLoad}>Load</button>
        </div>
        <div>
          <div>title</div>
          <input type="text" value={title} onChange={handleTitleChange} />
        </div>
        <div>
          <button onClick={handleCancel}>cancel</button>
          <button onClick={handleSave}>save</button>
        </div>
      </div>
    </Modal>
  );
};
