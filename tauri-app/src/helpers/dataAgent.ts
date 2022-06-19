import {Article, Article as ArticleModel, Channel, db} from '../db';

/**
 * 批量插入Article。检查是否存在，不存在则插入。
 * @param articles
 */
export const bulkAddArticle = (articles: ArticleModel[]) => {
  const links = articles.map((item: ArticleModel) => item.link)

  return db.articles.where("link")
    .anyOf(links)
    .toArray()
    .then((exists) => {
      if (exists.length < articles.length) {
        const remotes = articles.filter((item: Article) => {
          return !exists.some((exist) => exist.link === item.link);
        });

        if (remotes.length) {
          console.log('remotes', remotes);
          return db.transaction("rw", db.articles, async () => {
            await db.articles.bulkAdd(remotes);
          })
        }
      }

      return Promise.resolve()
    })
}

/**
 * 更新或者插入Channel
 * @param channel
 */
export const upsertChannel = async (channel: Channel) => {
  if (await db.channels.get({ feedUrl: channel.feedUrl })) {
    return db.channels.update(channel.feedUrl, channel);
  } else {
    return db.channels.put(channel);
  }
}

export const updateCountWithChannel = async (channel: Channel) => {
  const { feedUrl } = channel
  const c = await db.channels.get({ feedUrl })

  if (c) {
    const unread = await db.articles.where({
      feedUrl,
      unread: 1
    }).count()

    console.log('unread', unread)

    return { feedUrl: unread }
  }

  return {}
}
