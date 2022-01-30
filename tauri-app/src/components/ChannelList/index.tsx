import React, {useState, useEffect, useCallback, useContext} from 'react';
import {Icon} from '../Icon';
import styles from './channel.module.css';
import defaultSiteIcon from './default.png';
import {invoke} from "@tauri-apps/api/tauri";
import {useNavigate} from "react-router-dom";
import {RouteConfig} from "../../config";

const ChannelList = (): JSX.Element => {
  let navigate = useNavigate()
  const [channelList, setChannelList] = useState([]);
  const [currentId, setCurrentId] = useState('');
  const [sum, setSum] = useState(0);
  const [todayUnread, setTodayUnread] = useState(0);

  const initial = () => {
    invoke(`load_channels`).then((res) => {
      if (typeof res === "string") {
        setChannelList(JSON.parse(res));
      } else {
        setChannelList([]);
      }
    })
  };

  const refreshList = () => {
  };
  const viewArticles = async (channel: any) => {
    console.log(channel);
    navigate(
      `${RouteConfig.CHANNEL.replace(/:name/, channel.title)}?channelId=${
        channel.id
      }`
    );
  };

  const viewInbox = () => {
  };

  const goToSetting = () => {
  };

  const viewToday = () => {
  };

  const renderFeedList = (): JSX.Element => {
    return (
      <ul className={styles.list}>
        {channelList.map(
          (channel: any, i: number) => {
            const {articleCount = 0} = channel;

            return (
              <li
                className={`${styles.item} ${
                  currentId === channel.id ? styles.itemActive : ''
                }`}
                // eslint-disable-next-line react/no-array-index-key
                key={channel.title + i}
                onClick={() => viewArticles(channel)}
                aria-hidden="true"
              >
                <img
                  src={channel.favicon}
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
                {articleCount > 0 && (
                  <span className={styles.count}>{articleCount}</span>
                )}
              </li>
            );
          }
        )}
      </ul>
    );
  };

  useEffect(() => {
    initial();
  }, []);

  return (
    <div className={styles.container}>
      <div className={styles.header}>
        <div className={styles.toolbar}>
          <Icon name="add" customClass={styles.toolbarItem}/>
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
        <div className={styles.official}>
          <div
            className={styles.officialItem}
            aria-hidden="true"
            onClick={() => viewInbox()}
          >
            <Icon
              customClass={`${styles.officialItemIcon} ${styles.iconUnread}`}
              name="inbox"
            />
            <span className={styles.name}>所有文章</span>
            <span className={styles.count}>{sum}</span>
          </div>
          <div
            className={styles.officialItem}
            aria-hidden="true"
            onClick={() => viewToday()}
          >
            <Icon
              customClass={`${styles.officialItemIcon} ${styles.iconToday}`}
              name="calendar_today"
            />
            <span className={styles.name}>今日未读</span>
            <span className={styles.count}>{todayUnread}</span>
          </div>
        </div>
        {renderFeedList()}
      </div>
    </div>
  );
};

export {ChannelList};
