import React, {
    useCallback,
    useImperativeHandle,
    useMemo,
    useRef,
    useState,
  } from "react";
  import { Modal } from "../Modal";
  import { useModal } from "../Modal/useModal";
  import { http } from "@tauri-apps/api";
  
  export const AddFeedChannel = (props: any) => {
    const { showStatus, showModal, hideModal, toggleModal } = useModal();
    const [feedUrl, setFeedUrl] = useState("https://post.smzdm.com/feed");
    const [name, setName] = useState("");
    const [info, setInfo] = useState<any>({});
  
    const parseFeedXML = (xml: string) => {
      const parser = new DOMParser();
      const dom = parser.parseFromString(xml, "application/xml");
      console.log(dom);
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
            console.log(data);
            parseFeedXML(data);
          }
        });
    };
  
    const handleNameChange = (e: any) => {
      setName(e.target.name);
    };
  
    const handleInputChange = (e: any) => {
      setFeedUrl(e.target.value);
    };
  
    const handleSave = () => {};
  
    return (
      <Modal visible={showStatus} toggle={toggleModal} title="添加 RSS 订阅">
        <div>
          <div>
            <div>Feed URL</div>
            <input type="text" value={feedUrl} onChange={handleInputChange} />
            <button onClick={handleLoad}>Load</button>
          </div>
          <div>
            <div>name</div>
            <input type="text" value={name} onChange={handleNameChange} />
          </div>
          <div>
            <button onClick={handleSave}>save</button>
          </div>
        </div>
      </Modal>
    );
  };
  