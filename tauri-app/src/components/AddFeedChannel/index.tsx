import React, {useImperativeHandle, useState} from "react";
import {http} from "@tauri-apps/api";
import {Modal} from "../Modal";
import {useModal} from "../Modal/useModal";
import {db, Channel as ChannelModel, Article as ArticleModel} from "../../db";
import { parseFeedXML, extendChannel, extendFeedItems } from '../../helpers/parseXML';
import styles from './index.module.css';

export const AddFeedChannel = (props: any) => {
  const {showStatus, showModal, hideModal, toggleModal} = useModal();
  const [feedUrl, setFeedUrl] = useState("https://nodejs.org/en/feed/blog.xml");
  const [title, setTitle] = useState("");
  const [channel, setChannel] = useState({} as ChannelModel);
  const [articles, setArticles] = useState([] as ArticleModel[]);

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
      .then(({status, data}: any) => {
        if (status === 200) {
          const { channel, items } = parseFeedXML(data);

          setTitle(channel.title);
          setChannel(extendChannel(channel, { feedUrl }));
          setArticles(extendFeedItems(items, { feedUrl, unRead: 0 }));
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
    db.transaction('rw', db.channels, db.articles, async () => {
      db.channels.add(channel);
      db.articles.bulkAdd(articles);
    }).then(() => {
      toggleModal();
    })
  };

  return (
    <Modal visible={showStatus} toggle={toggleModal} title="添加 RSS 订阅" onConfirm={handleSave} onCancel={handleCancel}>
      <div className={styles.box}>
        <div className={styles.item}>
          <div className={styles.label}>Feed URL</div>
          <div className={styles.formItem}>
            <input type="text" value={feedUrl} onChange={handleInputChange}/>
          </div>
          <div className={styles.action}>
            <button onClick={handleLoad}>Load</button>
          </div>
        </div>

        <div className={styles.item}>
          <div className={styles.label}>Title</div>
          <div className={styles.formItem}>
            <input type="text" value={title} onChange={handleTitleChange}/>
          </div>
        </div>
      </div>
    </Modal>
  );
};
