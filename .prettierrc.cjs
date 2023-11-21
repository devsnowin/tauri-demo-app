/** @type {import("prettier").Config} */
const config = {
    trailingComma: 'es5',
    tabWidth: 4,
    singleQuote: true,
    plugins: ['prettier-plugin-tailwindcss'],
};

module.exports = config;
