import { useState } from "react";
import "./App.css";
import { open, save } from "@tauri-apps/plugin-dialog";
import { convertFileSrc, invoke } from "@tauri-apps/api/core";

const App = () => {
  const [errorMessage, setErrorMessage] = useState("");
  const [inputImagePath, setInputImagePath] = useState<string | null>(null);
  const [buffImagePath, setBuffImagePath] = useState<string | null>(null);
  const [message, setMessage] = useState("Hello, world!");
  const [hideMessage, setHideMessage] = useState("");
  const [bitsPerChannel, setBitsPerChannel] = useState(1);

  const OpenFile = async () => {
    const file = await open({
      multiple: false,
      directory: false,
    });
    if (file) {
      setInputImagePath(file);
      setErrorMessage("");
    }
  };

  const saveFile = async () => {
    if (buffImagePath) {
      let file = await save({
        filters: [
          {
            name: "Images",
            extensions: ["png"],
          },
        ],
      });
      if (file) {
        console.log(file);

        await invoke("save_file", { filepath: file });
        setErrorMessage("");
      } else {
        setErrorMessage("Failed to save the file.");
      }
    } else {
      setErrorMessage("Please encrypt the image first.");
    }
  };

  const hide = async () => {
    if (inputImagePath) {
      try {
        setBuffImagePath("");
        setBuffImagePath(
          await invoke("hide_data", {
            filepath: inputImagePath,
            message,
            bitsPerChannel,
          })
        );
        setErrorMessage("");
      } catch (error) {
        console.error(error);
        setErrorMessage("Failed to encrypt the image.");
      }
    } else {
      setErrorMessage("Please select an image first.");
    }
  };

  const reveal = async () => {
    if (inputImagePath) {
      try {
        setHideMessage(
          await invoke("extract_data", {
            filepath: inputImagePath,
            bitsPerChannel,
          })
        );
        setErrorMessage("");
      } catch (error) {
        console.error(error);
        setErrorMessage("Failed to decrypt the image.");
      }
    } else {
      setErrorMessage("Please select the image first.");
    }
  };

  return (
    <>
      <h2>Steganography</h2>
      {errorMessage && <div className="error"> {errorMessage} </div>}
      <div className="container">
        <div className="toolbar flex">
          <button className="icon-button" onClick={OpenFile}>
            Open file
          </button>
          <button className="icon-button" onClick={saveFile}>
            Save file
          </button>
        </div>
        <div className="image-comparison flex">
          <div className="image-container">
            {inputImagePath && (
              <img src={convertFileSrc(inputImagePath)} alt="Initial image" />
            )}
          </div>
          <div className="image-container">
            {buffImagePath && (
              <img src={convertFileSrc(buffImagePath)} alt="Changed image" />
            )}
          </div>
        </div>
        <div className="flex toolbar">
          <div className="generate-buttons">
            <button onClick={handleEncrypt} className="hide">
              hide data
            </button>
            <button onClick={reveal} className="generate-button">
              reveal data
            </button>
          </div>
        </div>
        <div className="flex">
          <input
            type="text"
            value={message}
            onChange={(e) => setMessage(e.target.value)}
          />
          <input
            type="number"
            value={bitsPerChannel}
            onChange={(e) => setBitsPerChannel(Number(e.target.value))}
            min="1"
            max="8"
          />
        </div>
        <div className="message">{hideMessage}</div>
      </div>
    </>
  );
};

export default App;
