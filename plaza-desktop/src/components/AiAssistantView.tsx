import React, { useState } from "react";
import { Sparkles, Send, Bot, Terminal, Cpu } from "lucide-react";
import { invoke } from "@tauri-apps/api/core";

export const AiAssistantView: React.FC = () => {
  const [prompt, setPrompt] = useState("");
  const [messages, setMessages] = useState<Array<{ sender: string; text: string }>>([
    {
      sender: "ai",
      text: "👋 Hello! I am Plaza AI, your workspace copilot. Tell me what workspace, runtime, or package profile you want to build (e.g. 'Create a PyTorch CUDA 12.4 workspace with Jupyter').",
    },
  ]);
  const [loading, setLoading] = useState(false);

  const handleSend = async () => {
    if (!prompt.trim()) return;
    const userText = prompt;
    setPrompt("");
    setMessages((prev) => [...prev, { sender: "user", text: userText }]);
    setLoading(true);

    try {
      const res = await invoke<string>("query_ai_assistant", { prompt: userText });
      setMessages((prev) => [...prev, { sender: "ai", text: res }]);
    } catch {
      setMessages((prev) => [
        ...prev,
        { sender: "ai", text: "Error connecting to Plaza AI engine." },
      ]);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="p-6 max-w-5xl mx-auto h-[calc(100vh-80px)] flex flex-col justify-between select-none">
      <div>
        <h2 className="text-2xl font-extrabold text-slate-100 tracking-tight flex items-center gap-3">
          <Sparkles className="w-6 h-6 text-amber-400" /> Plaza AI Assistant
        </h2>
        <p className="text-xs text-slate-400 mt-1">
          Natural language control for workspace creation, snapshot recommendations, and performance analysis
        </p>
      </div>

      <div className="flex-1 my-4 bg-slate-900/70 border border-slate-800/80 rounded-2xl p-6 overflow-y-auto space-y-4 shadow-inner">
        {messages.map((m, i) => (
          <div
            key={i}
            className={`flex items-start gap-3 ${
              m.sender === "user" ? "justify-end" : "justify-start"
            }`}
          >
            {m.sender === "ai" && (
              <div className="w-8 h-8 rounded-xl bg-amber-500/10 border border-amber-500/30 flex items-center justify-center text-amber-400 shrink-0">
                <Bot className="w-4 h-4" />
              </div>
            )}
            <div
              className={`max-w-xl p-4 rounded-2xl text-xs font-medium leading-relaxed ${
                m.sender === "user"
                  ? "bg-cyan-500 text-slate-950 rounded-br-none font-bold"
                  : "bg-slate-950/80 text-slate-200 border border-slate-800 rounded-bl-none"
              }`}
            >
              {m.text}
            </div>
          </div>
        ))}
        {loading && (
          <div className="flex items-center gap-2 text-xs text-amber-400 font-mono animate-pulse">
            <Sparkles className="w-4 h-4" /> Plaza AI is thinking...
          </div>
        )}
      </div>

      <div className="relative">
        <input
          type="text"
          placeholder="Ask Plaza AI to optimize workspace, rollback snapshot, or install packages..."
          value={prompt}
          onChange={(e) => setPrompt(e.target.value)}
          onKeyDown={(e) => e.key === "Enter" && handleSend()}
          className="w-full pl-5 pr-14 py-3.5 bg-slate-900 border border-slate-800 rounded-2xl text-xs text-slate-100 placeholder-slate-500 focus:outline-none focus:border-amber-500/50 shadow-xl"
        />
        <button
          onClick={handleSend}
          className="absolute right-2 top-2 p-2 bg-gradient-to-r from-amber-500 to-orange-500 text-slate-950 font-bold rounded-xl hover:opacity-90 transition active:scale-95 shadow-md"
        >
          <Send className="w-4 h-4" />
        </button>
      </div>
    </div>
  );
};
