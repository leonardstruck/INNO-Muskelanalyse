import type { AppProps } from "next/app";

import "../style.css";

import { Teko, Rubik } from "next/font/google";
import clsx from "clsx";
import { NextPage } from "next";
import MainLayout from "../components/layouts/main";
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { useEffect } from "react";

const rubik = Rubik({
  variable: "--font-rubik",
  subsets: ["latin"],
  weight: "variable"
})


export type NextPageWithLayout<P = {}, IP = P> = NextPage<P, IP> & {
  getLayout?: (page: React.ReactElement) => React.ReactNode
}

type AppPropsWithLayout = AppProps & {
  Component: NextPageWithLayout
}

const queryClient = new QueryClient()

export default function MyApp({ Component, pageProps }: AppPropsWithLayout) {
  // use the layout defined at the page level, if available
  const getLayout = Component.getLayout ?? ((page) => <MainLayout>{page}</MainLayout>)
  useEffect(() => {
    import("tauri-plugin-log-api").then((log) => {
      log.attachConsole().then((detach) => {
        console.log("attached console")
        return detach
      });
    })
  }, [])

  return (
    <>
      <style jsx global>
        {`
          :root {
            --font-rubik: ${rubik.style.fontFamily};
          }
        `}
      </style>
      <QueryClientProvider client={queryClient}>
        <div className="h-full font-sans">
          {getLayout(<Component {...pageProps} />)}
        </div>
      </QueryClientProvider>
    </>
  );
}
