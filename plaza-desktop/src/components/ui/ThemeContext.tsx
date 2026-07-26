import React, { createContext, useContext, useState, useEffect } from "react";

export type ThemeMode = "dark" | "light" | "high-contrast";

interface ThemeContextType {
  theme: ThemeMode;
  setTheme: (theme: ThemeMode) => void;
  toggleTheme: () => void;
}

const ThemeContext = createContext<ThemeContextType | undefined>(undefined);

export const ThemeProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [theme, setTheme] = useState<ThemeMode>(() => {
    const saved = localStorage.getItem("plazavm_theme");
    return (saved as ThemeMode) || "dark";
  });

  useEffect(() => {
    localStorage.setItem("plazavm_theme", theme);
    const root = document.documentElement;
    root.classList.remove("dark", "light", "high-contrast");
    root.classList.add(theme);
    root.setAttribute("data-theme", theme);
  }, [theme]);

  const toggleTheme = () => {
    setTheme((prev) => (prev === "dark" ? "light" : prev === "light" ? "high-contrast" : "dark"));
  };

  return (
    <ThemeContext.Provider value={{ theme, setTheme, toggleTheme }}>
      {children}
    </ThemeContext.Provider>
  );
};

export const useTheme = () => {
  const context = useContext(ThemeContext);
  if (!context) {
    throw new Error("useTheme must be used within a ThemeProvider");
  }
  return context;
};
