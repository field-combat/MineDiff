import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
/*

起動コマンド（4個）
d:
cd D:\work\self\差分書き出しアプリ\app\MineDiff
yarn
yarn tauri dev

https://gihyo.jp/article/2022/10/rust-monthly-topics-02

[2025.7.30] 先にUI作るか。
ディレクトリパスはテキストフィールドにして、コマンド投げるとこだけ、RUSTに渡す感じ。
そうすればRUSTはコマンド投げるやり方だけになるから調べる手間が減る。
で、形になったらディレクトリパスをRUST経由で取得できるようにすればええわ。
*/

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const[inputF, setInput] = useState("")
  const[outputF, setOutput] = useState("")
  const[dur, setDur] = useState("")

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    // let d = "/MAXAGE:" + dur;
    setGreetMsg(await invoke("greet", { name, inputF, outputF, dur }));
  }

  return (
    <main className="container">
      {/* <h1>Welcome to Tauri + React</h1> */}

      {/* <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div> */}
      {/* <p>Click on the Tauri, Vite, and React logos to learn more.</p> */}

      <form
        className=""
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
          className="form_name"
        />
        <div className="colmun">
        <label>対象のフォルダ：</label>
        <input id="select-input"
        onChange = {(e) => setInput(e.currentTarget.value)}
        placeholder="フォルダパス"
        />
        </div>
        <div className="colmun">
        <label>書き出し先：</label>
        <input id="select-output"
                 onChange = {(e) => setOutput(e.currentTarget.value)}
        placeholder="フォルダパス"/>
        
        </div>
        <div className="colmun">
        <label>更新日：</label>
        <input id="target-date" type="date" 
                 onChange = {(e) => setDur(e.currentTarget.value)}
        placeholder=""
        />
        </div>
        <button className="form_submit" type="submit">書き出し</button>
      </form>
      <p>{greetMsg}</p>
    </main>
  );
}

export default App;
