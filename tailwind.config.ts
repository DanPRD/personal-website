import type { Config } from "tailwindcss";

const config: Config = {
  content: ["./templates/*.html"],
  theme: {
    extend: {
      fontFamily: {
        pixel: ['nokia']
      }
    }
  },
  plugins: [],
}

export default config;

