/** @type {import('next').NextConfig} */
const nextConfig = {
  // Bazaar Tech: Force standard build logic to avoid root-inference mines
  typescript: {
    ignoreBuildErrors: true,
  },
  images: {
    unoptimized: true, // Crucial for Pi Browser compatibility
  },
  // This ensures the build is ready for the Pi App Studio (Static)
 // output: 'export', 
};

export default nextConfig;