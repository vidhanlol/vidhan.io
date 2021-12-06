/** @type {import('next').NextConfig} */
module.exports = {
  reactStrictMode: true,
  async rewrites() {
    return [
      {
        source: "/resume",
        destination: "https://resume.vidhan.io/resume",
      },
    ];
  },
};
