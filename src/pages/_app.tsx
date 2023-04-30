import type { AppProps } from "next/app";
import Layout from "../components/layout/Layout";

import "../style.css";

import { Teko } from "next/font/google";

const teko = Teko({
  variable: "--font-teko",
  subsets: ["latin"],
  weight: "400"
})

// This default export is required in a new `pages/_app.js` file.
export default function MyApp({ Component, pageProps }: AppProps) {
  return <div className={teko.variable}><Layout><Component {...pageProps} /></Layout></div>;
}
