use crate::common::errors::TantivySearchError;
use crate::logger::logger_bridge::TantivySearchLogger;
use crate::search::bridge::index_reader_bridge::IndexReaderBridge;
use crate::RowIdWithScore;
use crate::FFI_INDEX_SEARCHER_CACHE;
use crate::{common::constants::LOG_CALLBACK, ERROR};
use std::sync::Arc;

use super::strategy::query_strategy::BM25QueryStrategyWithColumnNames;
use super::strategy::query_strategy::QueryExecutor;

pub fn bm25_search_with_column_names(
    index_path: &str,
    sentence: &str,
    topk: u32,
    u8_aived_bitmap: &Vec<u8>,
    query_with_filter: bool,
    need_doc: bool,
    column_names: &Vec<String>,
) -> Result<Vec<RowIdWithScore>, TantivySearchError> {
    // Get index_reader_bridge from CACHE
    let index_reader_bridge: Arc<IndexReaderBridge> = FFI_INDEX_SEARCHER_CACHE
        .get_index_reader_bridge(index_path.to_string())
        .map_err(|e| {
            ERROR!(function:"bm25_search", "{}", e);
            TantivySearchError::InternalError(e)
        })?;

    // Choose query strategy to construct query executor.
    let sentence_query: BM25QueryStrategyWithColumnNames<'_> = BM25QueryStrategyWithColumnNames {
        sentence,
        topk: &topk,
        u8_aived_bitmap,
        query_with_filter: &query_with_filter,
        need_doc: &need_doc,
        column_names: &column_names,
    };

    let query_executor: QueryExecutor<'_, Vec<RowIdWithScore>> =
        QueryExecutor::new(&sentence_query);

    let searcher = &mut index_reader_bridge.reader.searcher();

    let result: Vec<RowIdWithScore> = query_executor.execute(searcher).map_err(
        |e: crate::common::errors::IndexSearcherError| {
            ERROR!(function:"bm25_search", "{}", e);
            TantivySearchError::IndexSearcherError(e)
        },
    )?;

    Ok(result)
}
