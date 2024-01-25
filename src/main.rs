use tfhe::integer::gen_keys_crt;
use tfhe::integer::wopbs::*;
use tfhe::shortint::parameters::parameters_wopbs_message_carry::WOPBS_PARAM_MESSAGE_4_CARRY_4_KS_PBS;
use tfhe::shortint::parameters::PARAM_MESSAGE_4_CARRY_4_KS_PBS;

fn main() {
    let basis: Vec<u64> = vec![13, 14, 15];

    let msg_space:u64 = basis.iter().copied().product();

    let (cks, sks) = gen_keys_crt(PARAM_MESSAGE_4_CARRY_4_KS_PBS, basis);
    let wopbs_key = WopbsKey::new_wopbs_key(&cks, &sks, &WOPBS_PARAM_MESSAGE_4_CARRY_4_KS_PBS);

    let clear = 42 % msg_space;
    let threshold = 30;
    let ct = cks.encrypt(clear);
    let ct = wopbs_key.keyswitch_to_wopbs_params(&sks, &ct);
    let lut = wopbs_key.generate_lut_crt(&ct, |x| if x > threshold {1} else {0});
    let ct_res = wopbs_key.wopbs(&ct, &lut);
    let ct_res = wopbs_key.keyswitch_to_pbs_params(&ct_res);
    let res = cks.decrypt(&ct_res);
    assert_eq!(res, if clear > threshold {1} else {0});
}
