import React from "react";
import ReactDOM from "react-dom";

type ModalProps = {
  title?: string;
  visible: boolean;
  toggle: any;
  children?: any;
};

const backdropStyles = {
  position: "fixed",
  top: 0,
  left: 0,
  width: "100%",
  height: "100%",
  zIndex: 10,
  transform: "translateZ(0)",
  backgroundColor: "rgba(0, 0, 0, 0.3)",
};

const modalStyles = {
  position: "fixed",
  padding: "2.5rem 1.5rem 1.5rem 1.5rem",
  backgroundColor: "white",
  boxShadow: "0 0 10px 3px rgba(0, 0, 0, 0.1)",
  overflowY: "auto",
  left: "50%",
  top: "50%",
  height: "auto",
  transform: "translate(-50%, -50%)",
  maxWidth: "30em",
  borderRadius: "0.25rem",
  maxHeight: "calc(100% - 1em)",
};

const Modal = (props: ModalProps) => {
  return props.visible
    ? ReactDOM.createPortal(
        //@ts-ignore
        <div style={backdropStyles}>
          {/* @ts-ignore */}
          <div className="modal" style={{ ...modalStyles }}>
            {props.children}
          </div>
        </div>,
        document.body
      )
    : null;
};

export { Modal };
