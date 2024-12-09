import { useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { convertFileSrc, invoke } from "@tauri-apps/api/core";

const App = () => {
  const [imagePath, setImagePath] = useState<string | null>(null);
  const [message, setMessage] = useState("Hello, world!");
  const [bitsPerChannel, setBitsPerChannel] = useState(1);

  const handleOpenFile = async () => {
    const file = await open({
      multiple: false,
      directory: false,
    });
    console.log(file);
    if (file) {
      setImagePath(file); // Сохраняем путь к изображению
    }
  };

  const handleEncrypt = async () => {
    if (imagePath) {
      try {
        await invoke("encrypt", {
          filepath: imagePath,
          message,
          bits_per_channel: bitsPerChannel,
        });
        alert("Изображение успешно зашифровано!");
      } catch (error) {
        console.error(error);
        alert("Ошибка при шифровании изображения.");
      }
    } else {
      alert("Сначала выберите файл для шифрования.");
    }
  };

  return (
    <div style={{ padding: "20px" }}>
      <h1>Шифрование изображения</h1>
      <button onClick={handleOpenFile}>Открыть файл</button>
      {imagePath && (
        <div>
          <h2>Выбранное изображение:</h2>
          <img
            src={convertFileSrc(imagePath)}
            alt="Selected"
            style={{ maxWidth: "300px", maxHeight: "300px" }}
          />
        </div>
      )}
      <div>
        <label>
          Сообщение:
          <input
            type="text"
            value={message}
            onChange={(e) => setMessage(e.target.value)}
          />
        </label>
      </div>
      <div>
        <label>
          Биты на канал:
          <input
            type="number"
            value={bitsPerChannel}
            onChange={(e) => setBitsPerChannel(Number(e.target.value))}
            min="1"
            max="8"
          />
        </label>
      </div>
      <button onClick={handleEncrypt}>Зашифровать</button>
    </div>
  );
};

export default App;

// import { useState } from "react";
// import { invoke } from "@tauri-apps/api/core";
// import "./App.css";

// function App() {
//   const [greetMsg, setGreetMsg] = useState("");
//   const [name, setName] = useState("");

//   async function greet() {
//     // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//     setGreetMsg(await invoke("greet", { name }));
//   }

//   return (
//     <main className="container">
//       //write simple two image comparison app
//       <form
//         className="row"
//         onSubmit={(e) => {
//           e.preventDefault();
//           greet();
//         }}
//       >
//         <input
//           id="greet-input"
//           onChange={(e) => setName(e.currentTarget.value)}
//           placeholder="Enter a name..."
//         />
//         <button type="submit">Greet</button>
//       </form>
//       <p>{greetMsg}</p>
//     </main>
//   );
// }

// export default App;
