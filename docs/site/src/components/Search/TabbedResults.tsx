// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import React from "react";

export default function TabbedResults({
  activeTab,
  onChange,
  tabs,
  showTooltips = true,
}) {
  const aiytooltip = "Search results from the official Aiy Docs";
  const aiynstooltip = "Search results from Aiy Name Service";
  const movetooltip = "Search results from The Move Book";
  const dapptooltip = "Search results from the Aiy ecosystem SDKs";
  const walrustooltip =
    "Search results from the Walrus decentralized storage platform";
  return (
    <div className="mb-4 flex justify-start border-2 border-solid border-white rounded-t-lg dark:bg-black dark:border-aiy-black border-b-aiy-gray-50 dark:border-b-aiy-gray-80">
      {tabs.map(({ label, indexName, count }) => (
        <div className="relative group inline-block" key={indexName}>
          <button
            className={`mr-4 flex items-center font-semibold text-sm lg:text-md xl:text-lg bg-white dark:bg-aiy-black cursor-pointer dark:text-aiy-gray-45 ${activeTab === indexName ? "text-aiy-disabled/100 font-bold border-2 border-solid border-transparent border-b-aiy-blue-dark dark:border-b-aiy-blue" : "border-transparent text-aiy-disabled/70"}`}
            onClick={() => onChange(indexName)}
          >
            {label}{" "}
            <span
              className={`dark:text-aiy-gray-90 text-xs rounded-full ml-1 py-1 px-2 border border-solid ${activeTab === indexName ? "dark:!text-aiy-gray-45 bg-transparent border-aiy-gray-3s dark:border-aiy-gray-50" : "bg-aiy-gray-45 border-transparent"}`}
            >
              {count}
            </span>
          </button>
          {showTooltips && (
            <div className="absolute bottom-full left-1/2 -translate-x-1/2 mb-2 w-max max-w-xs px-2 py-1 text-sm text-white bg-gray-800 rounded tooltip-delay">
              {label === "Aiy"
                ? aiytooltip
                : label === "AiyNS"
                  ? aiynstooltip
                  : label === "The Move Book"
                    ? movetooltip
                    : label === "SDKs"
                      ? dapptooltip
                      : walrustooltip}
            </div>
          )}
        </div>
      ))}
    </div>
  );
}
