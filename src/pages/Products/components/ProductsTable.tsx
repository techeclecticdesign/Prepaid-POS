import type Product from "../../../models/Product";

interface Props {
  products: Product[];
  onProductClick: (product: Product) => void;
}

export default function ProductsTable({ products, onProductClick }: Props) {
  return (
    <table className="min-w-full divide-y divide-gray-200">
      <thead>
        <tr>
          <th className="px-6 py-3 text-left text-xs 2xl:text-md font-medium text-gray-500 uppercase tracking-wider">
            Description
          </th>
          <th className="px-6 py-3 text-left text-xs 2xl:text-md font-medium text-gray-500 uppercase tracking-wider">
            Category
          </th>
          <th className="px-6 py-3 text-right text-xs 2xl:text-md font-medium text-gray-500 uppercase tracking-wider">
            Price
          </th>
        </tr>
      </thead>
      <tbody className="bg-white divide-y divide-gray-200">
        {products.map((p) => (
          <tr
            key={p.upc}
            className="hover:bg-gray-50 cursor-pointer"
            onClick={() => onProductClick(p)}
          >
            <td className="px-6 py-2 2xl:py-5 whitespace-nowrap text-sm 2xl:text-xl text-gray-900">
              {p.desc}
            </td>
            <td className="px-6 py-2 2xl:py-5 whitespace-nowrap text-sm 2xl:text-xl text-gray-900">
              {p.category}
            </td>
            <td className="px-6 py-2 2xl:py-5 whitespace-nowrap text-sm 2xl:text-xl text-gray-900 text-right">
              ${(p.price / 100).toFixed(2)}
            </td>
          </tr>
        ))}
      </tbody>
    </table>
  );
}
