import path from 'path';

const src = path.resolve(__dirname, 'src');
const dst = path.resolve(__dirname, 'dst');

export default {

  entry: {
    main: path.resolve(src, 'main.tsx'),
  },

  output: {
    path: dst,
    filename: '[name].bundle.js',
  },

  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      }
    ]
  },

  resolve: {
    extensions: ['.tsx', '.ts', '.js'],
  },

}
