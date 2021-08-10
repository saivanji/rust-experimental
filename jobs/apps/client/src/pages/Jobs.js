import { useState, useRef, cloneElement } from "react";
import cx from "classnames";
import {
  useQueryParam,
  StringParam,
  NumberParam,
  withDefault,
} from "use-query-params";
import { useQuery } from "react-query";
import sanitizeHtml from "sanitize-html";
import { ReactComponent as Bookmark } from "assets/bookmark.svg";
import { ReactComponent as CheckDouble } from "assets/check-double.svg";
import { ReactComponent as Close } from "assets/close.svg";
import { ReactComponent as ExternalLink } from "assets/external-link.svg";
import * as api from "api.js";

export default function Jobs() {
  const [page, setPage] = useQueryParam("page", withDefault(NumberParam, 1));
  const [section, setSection] = useQueryParam(
    "section",
    withDefault(StringParam, "all")
  );
  const [opened, setOpened] = useState(null);

  const limit = 15;
  const offset = (page - 1) * limit;

  const jobs = useQuery(["jobs", limit, offset], () =>
    api.listJobs(limit, offset)
  );

  const pagination = (
    <Pagination
      page={page}
      limit={limit}
      total={jobs.data?.data.total}
      onChange={setPage}
    />
  );

  return (
    <div className="p-3 max-w-3xl w-full mx-auto">
      {opened ? (
        <Content {...opened} onClose={() => setOpened(null)} />
      ) : (
        <div className="pb-4">
          <Tabs selected={section} onChange={setSection} />
          {pagination}
          <div className="my-2">
            {jobs.isLoading
              ? "Loading..."
              : jobs.data.data.items.map((item) => {
                  return (
                    <Job
                      key={item.id}
                      {...item}
                      onClick={() => setOpened(item)}
                    />
                  );
                })}
          </div>
          {pagination}
        </div>
      )}
    </div>
  );
}

function Job({ title, source, remote, tags, onClick }) {
  return (
    <>
      <div
        onClick={onClick}
        className="p-3 -mx-3 rounded hover:bg-gray-100 cursor-pointer"
      >
        <div className="flex items-center">
          <h2 className="font-semibold text-md mb-1">{title}</h2>
          <span className="text-xs ml-auto text-gray-400">
            {source}, remote - {remote}
          </span>
        </div>
        <div className="flex flex-wrap items-center">
          <Tags items={tags} />
        </div>
      </div>
    </>
  );
}

function Tags({ items }) {
  return items.map((tag, i) => (
    <span
      key={i}
      className="rounded p-1 text-xs bg-gray-200 text-gray-700 mr-2"
    >
      {tag}
    </span>
  ));
}

function Tabs({ selected, onChange }) {
  return (
    <div className="text-xs mb-5 border-b border-gray-200">
      <TabButton name="all" selected={selected} onClick={onChange}>
        All
      </TabButton>
      <TabButton name="saved" selected={selected} onClick={onChange}>
        Saved
      </TabButton>
    </div>
  );
}

function TabButton({ children, selected, name, onClick }) {
  const isSelected = selected === name;

  return (
    <button
      className={cx("px-3 py-2 rounded-t", isSelected && "bg-gray-100")}
      onClick={() => onClick(name)}
    >
      {children}
    </button>
  );
}

function Content({ url, title, description, tags, onClose }) {
  const sanitized = sanitizeHtml(description);
  const ref = useRef();

  return (
    <div ref={ref}>
      <div className="flex items-start mb-2">
        <h1 className="font-semibold text-2xl">{title}</h1>
        <div className="ml-auto flex items-center">
          <Action onClick={() => window.open(url)}>
            <ExternalLink />
          </Action>
          <Action>
            <Bookmark />
          </Action>
          <Action>
            <CheckDouble />
          </Action>
          <Action onClick={onClose}>
            <Close />
          </Action>
        </div>
      </div>
      <Tags items={tags} />
      <div
        className="mt-4"
        dangerouslySetInnerHTML={{
          __html: sanitized,
        }}
      />
    </div>
  );
}

function Action({ children, onClick }) {
  return (
    <button
      onClick={onClick}
      className="ml-2 p-2 rounded w-8 hover:bg-gray-100"
    >
      {cloneElement(children, { className: "w-full h-full" })}
    </button>
  );
}

function Pagination({ page, total, limit, onChange }) {
  const prevDisabled = page === 1;
  const nextDisabled = page === Math.ceil(total / limit);

  return (
    <div className="flex text-md justify-between items-center">
      <PaginationButton
        disabled={prevDisabled}
        onClick={() => onChange(page - 1)}
      >
        Prev
      </PaginationButton>
      <span className="text-xs">{page}</span>
      <PaginationButton
        disabled={nextDisabled}
        onClick={() => onChange(page + 1)}
      >
        Next
      </PaginationButton>
    </div>
  );
}

function PaginationButton({ children, className, disabled, onClick }) {
  return (
    <button
      disabled={disabled}
      className={cx(
        "font-semibold",
        className,
        disabled && "text-gray-400 cursor-not-allowed"
      )}
      onClick={onClick}
    >
      {children}
    </button>
  );
}
