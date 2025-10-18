import React from "react";

const Sidebar: React.FC = () => {
  return (
    <aside className="w-64 bg-gray-100 p-4 border-r border-gray-200 dark:bg-gray-800 dark:border-gray-700">
      <h2 className="text-lg font-semibold mb-4">Folders</h2>
      {/* Folder tree will go here */}
      <ul>
        <li className="p-2 rounded-md hover:bg-gray-200 dark:hover:bg-gray-700 cursor-pointer">
          All Photos
        </li>
        <li className="p-2 rounded-md hover:bg-gray-200 dark:hover:bg-gray-700 cursor-pointer">
          Favorites
        </li>
      </ul>
    </aside>
  );
};

export default Sidebar;
