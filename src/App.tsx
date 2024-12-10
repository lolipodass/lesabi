import { useState } from "react";
import "./App.css";
import { open } from "@tauri-apps/plugin-dialog";
import { convertFileSrc, invoke } from "@tauri-apps/api/core";

const App = () => {
  const [errorMessage, setErrorMessage] = useState("");
  const [inputImagePath, setInputImagePath] = useState<string | null>(null);
  const [buffImagePath, setBuffImagePath] = useState<string | null>(null);
  const [message, setMessage] = useState("Hello, world!");
  const [bitsPerChannel, setBitsPerChannel] = useState(1);

  const handleOpenFile = async () => {
    const file = await open({
      multiple: false,
      directory: false,
    });
    if (file) {
      setInputImagePath(file);
      setErrorMessage("");
    }
  };

  const handleEncrypt = async () => {
    if (inputImagePath) {
      try {
        setBuffImagePath(
          await invoke("encrypt", {
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

  return (
    <>
      <h2>Steganography</h2>
      {errorMessage && <div className="error"> {errorMessage} </div>}
      <div className="container">
        <div className="toolbar flex">
          <button className="icon-button" onClick={handleOpenFile}>
            Open file
          </button>
          <button className="icon-button">Сохранить файл</button>
        </div>
        <div className="image-comparison flex">
          <div className="image-container">
            {inputImagePath && (
              <img src={convertFileSrc(inputImagePath)} alt="Изображение 1" />
            )}
          </div>
          <div className="image-container">
            {buffImagePath && (
              <img src={convertFileSrc(buffImagePath)} alt="Изображение 2" />
            )}
          </div>
        </div>
        <div className="flex toolbar">
          <div className="generate-buttons">
            <button onClick={handleEncrypt} className="hide">
              hide data
            </button>
            <button onClick={handleEncrypt} className="generate-button">
              reveal data
            </button>
          </div>
        </div>
        <div>
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
      </div>
    </>
  );
};

export default App;
