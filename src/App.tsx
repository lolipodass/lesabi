import { useEffect, useState } from "react";
import "./App.css";
import { open, save } from "@tauri-apps/plugin-dialog";
import { convertFileSrc, invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

const App = () => {
  const [errorMessage, setErrorMessage] = useState("");
  const [inputImagePath, setInputImagePath] = useState<string | null>(null);
  const [buffImagePath, setBuffImagePath] = useState<string | null>(null);
  const [message, setMessage] = useState("Hello, world!");
  const [hideMessage, setHideMessage] = useState("");
  const [bitsPerChannel, setBitsPerChannel] = useState(1);
  const [hideFunctionDuration, setHideFunctionDuration] = useState(0);
  const [extractFunctionDuration, setExtractFunctionDuration] = useState(0);

  useEffect(() => {
    const hideTime = () => {
      listen("hide_function_duration", (event: any) => {
        console.log(event);

        setHideFunctionDuration(event.payload);
      });
    };
    hideTime();

    const extractTime = () => {
      listen("extract_function_duration", (event: any) => {
        console.log(event);

        setExtractFunctionDuration(event.payload);
      });
    };

    extractTime();
  }, []);

  const OpenFile = async () => {
    const file = await open({
      multiple: false,
      directory: false,
      filters: [
        {
          name: "Images",
          extensions: [
            "ff",
            "gif",
            "hdr",
            "ico",
            "jpg",
            "jpeg",
            "exr",
            "png",
            "ppm",
            "pgm",
            "pbm",
            "qoi",
            "tga",
            "tif",
            "tiff",
          ],
        },
      ],
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
        try {
          await invoke("save_file", { filepath: file });
          setErrorMessage("");
        } catch (error: any) {
          setErrorMessage(error);
        }
      } else {
        setErrorMessage("Failed to save the file.");
      }
    } else {
      setErrorMessage("Please encrypt the image first.");
    }
  };

  const hide = async () => {
    if (!message.trim()) {
      setErrorMessage("Message cannot be empty.");
      return;
    }

    if (bitsPerChannel < 1 || bitsPerChannel > 8) {
      setErrorMessage("Bits per channel must be between 1 and 8.");
      return;
    }

    if (inputImagePath) {
      try {
        setBuffImagePath(
          await invoke("hide_data", {
            filepath: inputImagePath,
            message,
            bitsPerChannel,
          })
        );
        setErrorMessage("");
      } catch (error: any) {
        console.error(error);
        setErrorMessage(`Failed to encrypt the image: ${error}`);
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
              <img
                src={`${convertFileSrc(buffImagePath)}?${Date.now()}`}
                alt="Changed image"
              />
            )}
          </div>
        </div>
        <div className="flex toolbar">
          <button onClick={hide} className="hide">
            hide data
          </button>
          <button onClick={reveal} className="generate-button">
            reveal data
          </button>
        </div>
        <div className="flex toolbar">
          <div>time to hide: {hideFunctionDuration}, ms</div>
          <div>time to reveal: {extractFunctionDuration}, ms</div>
        </div>
        <div className="flex toolbar">
          <textarea
            placeholder="Message"
            value={message}
            onChange={(e) => setMessage(e.target.value)}
          />
          <input
            type="number"
            placeholder="Bits per channel"
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
