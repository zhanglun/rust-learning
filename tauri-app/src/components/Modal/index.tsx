import React from "react";
import ReactDOM from "react-dom";

type ModalProps = {
  title?: string;
  visible: boolean;
  toggle: any;
  children?: any;
};

const Modal = (props: ModalProps) => {
  const { visible, title, children } = props;

  return visible
    ? ReactDOM.createPortal(
        //@ts-ignore
        <div className="modal-overlay">
          {/* @ts-ignore */}
          <div className="modal">
            <div className="modal-header">
              { title ? <div className="modal-title">{title}</div> : null}
            </div>
            {children}
          </div>
        </div>,
        document.body
      )
    : null;
};

export { Modal };
