import { type AppProps } from "$fresh/server.ts";

export default function App({ Component }: AppProps) {
  return (
    <html lang="th">
      <head>
        <meta charSet="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <meta name="color-scheme" content="light dark" />
        <title>Nomkhonwaan</title>
        <meta
          name="description"
          content="Trust me I'm Petdo"
        />
        <link rel="icon" href="/favicon.ico" />
        <link rel="preconnect" href="https://fonts.googleapis.com" />
        <link rel="preconnect" href="https://fonts.gstatic.com" crossOrigin="" />
        <link
          href="https://fonts.googleapis.com/css2?family=Maitree:wght@400;700&family=Prompt:wght@700&family=Source+Code+Pro&display=swap"
          rel="stylesheet"
        />
        <link rel="stylesheet" href="/styles.css" />

        {/* Google Analytics */}
        <script
          async
          src="https://www.googletagmanager.com/gtag/js?id=G-ER347CPNY4"
        >
        </script>
        <script
          dangerouslySetInnerHTML={{
            __html: `
              window.dataLayer = window.dataLayer || [];
              function gtag(){dataLayer.push(arguments);}
              gtag('js', new Date());
              gtag('config', 'G-ER347CPNY4');
            `,
          }}
        />
      </head>
      <body>
        <Component />
      </body>
    </html>
  );
}
