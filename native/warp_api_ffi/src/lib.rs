use allo_isolate::ffi;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

mod api;

static mut POST_COBJ: Option<ffi::DartPostCObjectFnType> = None;

#[no_mangle]
pub unsafe extern "C" fn init_wallet(db_path: *mut c_char, ld_url: *mut c_char) {
    let db_path = CStr::from_ptr(db_path).to_string_lossy();
    let ld_url = CStr::from_ptr(ld_url).to_string_lossy();
    api::init_wallet(&db_path, &ld_url);
}

#[no_mangle]
pub unsafe extern "C" fn warp_sync(get_tx: bool, anchor_offset: u32, port: i64) {
    api::warp_sync(get_tx, anchor_offset, port);
}

#[no_mangle]
pub unsafe extern "C" fn dart_post_cobject(ptr: ffi::DartPostCObjectFnType) {
    POST_COBJ = Some(ptr);
}

#[no_mangle]
pub unsafe extern "C" fn get_latest_height() -> u32 {
    api::get_latest_height()
}

#[no_mangle]
pub unsafe extern "C" fn is_valid_key(seed: *mut c_char) -> i8 {
    let seed = CStr::from_ptr(seed).to_string_lossy();
    sync::is_valid_key(&seed)
}

#[no_mangle]
pub unsafe extern "C" fn valid_address(address: *mut c_char) -> bool {
    let address = CStr::from_ptr(address).to_string_lossy();
    api::valid_address(&address)
}

#[no_mangle]
pub unsafe extern "C" fn new_address(account: u32) -> *mut c_char {
    let address = api::new_address(account);
    CString::new(address).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn set_mempool_account(account: u32) {
    api::set_mempool_account(account);
}

#[no_mangle]
pub unsafe extern "C" fn new_account(name: *mut c_char, data: *mut c_char) -> i32 {
    let name = CStr::from_ptr(name).to_string_lossy();
    let data = CStr::from_ptr(data).to_string_lossy();
    api::new_account(&name, &data)
}

#[no_mangle]
pub unsafe extern "C" fn get_mempool_balance() -> i64 {
    api::get_mempool_balance()
}

#[no_mangle]
pub unsafe extern "C" fn send_multi_payment(
    account: u32,
    recipients_json: *mut c_char,
    anchor_offset: u32,
    use_transparent: bool,
    port: i64,
) -> *const c_char {
    let recipients_json = CStr::from_ptr(recipients_json).to_string_lossy();
    let tx_id = api::send_multi_payment(
        account,
        &recipients_json,
        use_transparent,
        anchor_offset,
        port,
    );
    CString::new(tx_id).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn try_warp_sync(get_tx: bool, anchor_offset: u32) -> i8 {
    api::try_warp_sync(get_tx, anchor_offset)
}

#[no_mangle]
pub unsafe extern "C" fn skip_to_last_height() {
    api::skip_to_last_height()
}

#[no_mangle]
pub unsafe extern "C" fn rewind_to_height(height: u32) {
    api::rewind_to_height(height)
}

#[no_mangle]
pub unsafe extern "C" fn mempool_sync() -> i64 {
    api::mempool_sync()
}

#[no_mangle]
pub unsafe extern "C" fn mempool_reset(height: u32) {
    api::mempool_reset(height)
}

#[no_mangle]
pub unsafe extern "C" fn get_taddr_balance(account: u32) -> u64 {
    api::get_taddr_balance(account)
}

#[no_mangle]
pub unsafe extern "C" fn shield_taddr(account: u32) -> *mut c_char {
    let tx_id = api::shield_taddr(account);
    CString::new(tx_id).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn set_lwd_url(url: *mut c_char) {
    let url = CStr::from_ptr(url).to_string_lossy();
    api::set_lwd_url(&url);
}

#[no_mangle]
pub unsafe extern "C" fn prepare_multi_payment(
    account: u32,
    recipients_json: *mut c_char,
    use_transparent: bool,
    anchor_offset: u32,
) -> *mut c_char {
    let recipients_json = CStr::from_ptr(recipients_json).to_string_lossy();
    let tx = api::prepare_multi_payment(account, &recipients_json, use_transparent, anchor_offset);
    CString::new(tx).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn broadcast(tx_filename: *mut c_char) -> *mut c_char {
    let tx_filename = CStr::from_ptr(tx_filename).to_string_lossy();
    let res = api::broadcast(&tx_filename);
    CString::new(res).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn broadcast_txhex(txhex: *mut c_char) -> *mut c_char {
    let txhex = CStr::from_ptr(txhex).to_string_lossy();
    let res = api::broadcast_txhex(&txhex);
    CString::new(res).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn sync_historical_prices(now: i64, days: u32, currency: *mut c_char) -> u32 {
    let currency = CStr::from_ptr(currency).to_string_lossy();
    api::sync_historical_prices(now, days, &currency)
}

#[no_mangle]
pub unsafe extern "C" fn get_ua(
    sapling_addr: *mut c_char,
    transparent_addr: *mut c_char,
) -> *mut c_char {
    let sapling_addr = CStr::from_ptr(sapling_addr).to_string_lossy();
    let transparent_addr = CStr::from_ptr(transparent_addr).to_string_lossy();
    let ua_addr = api::get_ua(&sapling_addr, &transparent_addr);
    CString::new(ua_addr).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn get_sapling(ua_addr: *mut c_char) -> *mut c_char {
    let ua_addr = CStr::from_ptr(ua_addr).to_string_lossy();
    let sapling_addr = api::get_sapling(&ua_addr);
    CString::new(sapling_addr).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn store_contact(
    id: u32,
    name: *mut c_char,
    address: *mut c_char,
    dirty: bool,
) {
    let name = CStr::from_ptr(name).to_string_lossy();
    let address = CStr::from_ptr(address).to_string_lossy();
    api::store_contact(id, &name, &address, dirty);
}

#[no_mangle]
pub unsafe extern "C" fn commit_unsaved_contacts(account: u32, anchor_offset: u32) -> *mut c_char {
    let tx_id = api::commit_unsaved_contacts(account, anchor_offset);
    CString::new(tx_id).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn delete_account(account: u32) {
    api::delete_account(account);
}

#[no_mangle]
pub unsafe extern "C" fn truncate_data() {
    api::truncate_data();
}

#[no_mangle]
pub unsafe extern "C" fn make_payment_uri(
    address: *mut c_char,
    amount: u64,
    memo: *mut c_char,
) -> *mut c_char {
    let address = CStr::from_ptr(address).to_string_lossy();
    let memo = CStr::from_ptr(memo).to_string_lossy();
    let uri = api::make_payment_uri(&address, amount, &memo);
    CString::new(uri).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn parse_payment_uri(uri: *mut c_char) -> *mut c_char {
    let uri = CStr::from_ptr(uri).to_string_lossy();
    let payment_json = api::parse_payment_uri(&uri);
    CString::new(payment_json).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn store_share_secret(account: u32, secret: *mut c_char) {
    let secret = CStr::from_ptr(secret).to_string_lossy();
    api::store_share_secret(account, &secret);
}

#[no_mangle]
pub unsafe extern "C" fn get_share_secret(account: u32) -> *mut c_char {
    let secret = api::get_share_secret(account);
    CString::new(secret).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn run_aggregator(secret_share: *mut c_char, port: u16, send_port: i64) {
    let secret_share = CStr::from_ptr(secret_share).to_string_lossy();
    api::run_aggregator(&secret_share, port, send_port);
}

#[no_mangle]
pub unsafe extern "C" fn shutdown_aggregator() {
    api::shutdown_aggregator();
}

#[no_mangle]
pub unsafe extern "C" fn submit_multisig_tx(tx_json: *mut c_char, port: u16) -> *mut c_char {
    let tx_json = CStr::from_ptr(tx_json).to_string_lossy();
    let raw_tx = api::submit_multisig_tx(&tx_json, port);
    let raw_tx = hex::encode(raw_tx);
    CString::new(raw_tx).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn run_multi_signer(
    address: *mut c_char,
    amount: u64,
    secret_share: *mut c_char,
    aggregator_url: *mut c_char,
    my_url: *mut c_char,
    port: u16,
) -> u32 {
    let address = CStr::from_ptr(address).to_string_lossy();
    let secret_share = CStr::from_ptr(secret_share).to_string_lossy();
    let aggregator_url = CStr::from_ptr(aggregator_url).to_string_lossy();
    let my_url = CStr::from_ptr(my_url).to_string_lossy();
    api::run_multi_signer(
        &address,
        amount,
        &secret_share,
        &aggregator_url,
        &my_url,
        port,
    )
}

#[no_mangle]
pub unsafe extern "C" fn split_account(
    threshold: u32,
    participants: u32,
    account: u32,
) -> *mut c_char {
    let r = api::split_account(threshold, participants, account);
    CString::new(r).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn dummy_export() {}
