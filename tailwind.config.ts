import type { Config } from "tailwindcss";

const config: Config = {
  content: ["./templates/*.html"],
  theme: {
    extend: {
      fontFamily: {
        pixel: ['nokia'],
        w95: ['w95fa'],
        vv: ['vv']
      }
    }
  },
  plugins: [],
}

export default config;

