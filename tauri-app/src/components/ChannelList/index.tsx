import React, {useState, useEffect, useRef} from "react";
import {NavLink} from "react-router-dom";
import {useLiveQuery} from "dexie-react-hooks";
import {Icon} from "../Icon";
import styles from "./channel.module.css";
import defaultSiteIcon from "./default.png";
import {useNavigate} from "react-router-dom";
import {RouteConfig} from "../../config";
import {db} from "../../db";
import {AddFeedChannel} from "../AddFeedChannel";
import {Toast} from "../Toast";
import {getFavico, requestFeed} from "../../helpers/parseXML";
import * as dataAgent from '../../helpers/dataAgent';
import {updateCountWithChannel} from "../../helpers/dataAgent";

const ChannelList = (props: any): JSX.Element => {
  const channelList = useLiveQuery(() => db.channels.toArray(), []);
  const navigate = useNavigate();
  const addFeedButtonRef = useRef(null);
  const [currentId, setCurrentId] = useState("");
  const [sum, setSum] = useState(0);
  const [todayUnread, setTodayUnread] = useState(0);

  const initial = () => {
  };
  const loadAndUpdate = (url: string) => {
    return requestFeed(url).then(async (res) => {
      if (res.channel && res.items) {
        const {channel, items} = res;

        db.transaction("rw", db.channels, db.articles, async () => {
          await dataAgent.bulkAddArticle(items)
          await dataAgent.upsertChannel(channel)
          await dataAgent.updateCountWithChannel(channel.feedUrl)
        });
      }

      return res;
    });
  };

  const refreshList = () => {
    Toast.show({
      type: "success",
      title: "正在同步",
      content: "同步所有订阅，可能会花一小段时间，请稍候",
    });

    const urlList = (channelList || []).map((channel) => {
      return channel.feedUrl;
    });

    const limit = 5;
    let cur = 0;
    let tasks: Promise<any>[] = [];
    const res: Promise<any>[] = [];
    const enQueue = (): Promise<any> => {
      if (cur === urlList?.length || urlList.length === 0) {
        return Promise.resolve();
      }

      const url = urlList[cur];

      cur += 1;

      const p = Promise.resolve().then(() => loadAndUpdate(url));

      res.push(p);

      let r = Promise.resolve();

      if (limit <= urlList.length) {
        const e: Promise<any> = p.then(() => tasks.splice(tasks.indexOf(e), 1));
        tasks.push(e);
        if (tasks.length >= limit) {
          r = Promise.race(tasks);
        }
      }

      return r.then(() => enQueue());
    };

    enQueue().then(() => {
      return Promise.all(res);
    }).then(() => {
      Toast.show({
        type: "success",
        title: "同步完成",
      });
    });
  };

  const viewInbox = () => {
  };

  const goToSetting = () => {
    navigate(RouteConfig.SETTINGS);
  };

  const viewToday = () => {
  };

  const renderFeedList = (): JSX.Element => {
    return (
      <ul className={styles.list}>
        {channelList?.map((channel: any, i: number) => {
          const {unread = 0, link} = channel;
          const ico = getFavico(link);

          return (
            <li
              key={channel.title + i}
              aria-hidden="true"
            >
              <NavLink
                className={({isActive}) => `${styles.item} ${
                  isActive ? styles.itemActive : ""
                }`}
                to={`${RouteConfig.CHANNEL.replace(/:name/, encodeURI(channel.title))}?channelId=${
                  channel.id
                }&feedUrl=${channel.feedUrl}`}>
                <img
                  src={ico}
                  onError={(e) => {
                    // @ts-ignore
                    e.target.onerror = null;

                    // @ts-ignore
                    e.target.src = defaultSiteIcon;
                  }}
                  className={styles.icon}
                  alt={channel.title}
                />
                <span className={styles.name}>{channel.title}</span>
                {unread > 0 && (
                  <span className={styles.count}>{unread}</span>
                )}
              </NavLink>
            </li>
          );
        })}
      </ul>
    );
  };

  const addFeed = () => {
    if (addFeedButtonRef && addFeedButtonRef.current) {
      (addFeedButtonRef.current as any).showModal();
    }
  };

  useEffect(() => {
    initial();
  }, []);

  return (
    <div className={styles.container}>
      <div className={styles.header}>
        <div className={styles.toolbar}>
          <AddFeedChannel Aref={addFeedButtonRef}/>
          <Icon name="add" customClass={styles.toolbarItem} onClick={addFeed}/>
          <Icon name="folder" customClass={styles.toolbarItem}/>
          <Icon
            name="refresh"
            customClass={styles.toolbarItem}
            onClick={refreshList}
          />
          <Icon
            name="settings"
            customClass={styles.toolbarItem}
            onClick={goToSetting}
            aria-hidden="true"
          />
        </div>
      </div>
      <div className={styles.inner}>
        {/*<div className={styles.official}>*/}
        {/*  <div*/}
        {/*    className={styles.officialItem}*/}
        {/*    aria-hidden="true"*/}
        {/*    onClick={() => viewInbox()}*/}
        {/*  >*/}
        {/*    <Icon*/}
        {/*      customClass={`${styles.officialItemIcon} ${styles.iconUnread}`}*/}
        {/*      name="inbox"*/}
        {/*    />*/}
        {/*    <span className={styles.name}>所有文章</span>*/}
        {/*    <span className={styles.count}>{sum}</span>*/}
        {/*  </div>*/}
        {/*  <div*/}
        {/*    className={styles.officialItem}*/}
        {/*    aria-hidden="true"*/}
        {/*    onClick={() => viewToday()}*/}
        {/*  >*/}
        {/*    <Icon*/}
        {/*      customClass={`${styles.officialItemIcon} ${styles.iconToday}`}*/}
        {/*      name="calendar_today"*/}
        {/*    />*/}
        {/*    <span className={styles.name}>今日未读</span>*/}
        {/*    <span className={styles.count}>{todayUnread}</span>*/}
        {/*  </div>*/}
        {/*</div>*/}
        {renderFeedList()}
      </div>
    </div>
  );
};

export {ChannelList};
