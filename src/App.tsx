import { useEffect, useState } from "react";
import "./App.css";
import { open, save } from "@tauri-apps/plugin-dialog";
import { convertFileSrc, invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

const App = () => {
  const [errorMessage, setErrorMessage] = useState("");
  const [inputImagePath, setInputImagePath] = useState<string | null>(null);
  const [inputMatrixPath, setInputMatrixPath] = useState<string>("");
  const [buffImagePath, setBuffImagePath] = useState<string | null>(null);
  const [buffMatrixPath, setBuffMatrixPath] = useState<string>("");
  const [message, setMessage] = useState("Hello, world!");
  const [hideMessage, setHideMessage] = useState("");
  const [bitsPerChannel, setBitsPerChannel] = useState(1);
  const [checked, setChecked] = useState(false);
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
      if (checked) {
        generateMap();
      }
      setErrorMessage("");
    } else {
      setErrorMessage("Failed to open the file.");
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
        if (checked) {
          generateMap();
        }
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

  const generateMap = async () => {
    if (inputImagePath || buffImagePath) {
      if (inputImagePath) {
        try {
          setInputMatrixPath(
            await invoke("generate_map", {
              filepath: inputImagePath,
              name: "input",
            })
          );
          setErrorMessage("");
        } catch (error) {
          console.error(error);
          setErrorMessage("Failed to decrypt the image: " + error);
        }
      }
      if (buffImagePath) {
        try {
          setBuffMatrixPath(
            await invoke("generate_map", {
              filepath: buffImagePath,
              name: "buff",
            })
          );
          setErrorMessage("");
        } catch (error) {
          console.error(error);
          setErrorMessage("Failed to decrypt the image: " + error);
        }
      }
    } else {
      setErrorMessage("Please select at least one image first.");
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
              <img
                src={`${convertFileSrc(
                  checked ? inputMatrixPath : inputImagePath
                )}?${Date.now()}`}
                alt="Initial image"
              />
            )}
          </div>
          <div className="image-container">
            {buffImagePath && (
              <img
                src={`${convertFileSrc(
                  checked ? buffMatrixPath : buffImagePath
                )}?${Date.now()}`}
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
          <div>
            <input
              id="image_matrix"
              type="checkbox"
              checked={checked}
              onChange={() => setChecked(!checked)}
              onClick={generateMap}
              className="image_matrix"
            />
            <label htmlFor="image_matrix">image map</label>
          </div>
        </div>
        <div className="message">{hideMessage}</div>
      </div>
    </>
  );
};

export default App;
