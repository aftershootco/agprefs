use nu_plugin::{EvaluatedCall, LabeledError, Plugin};
use nu_protocol::{Category, PluginSignature, Record, Type};
pub struct Agp;

const CALL_SIGNATURE: &str = "from agp";
impl Plugin for Agp {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![PluginSignature::build(CALL_SIGNATURE)
            .input_output_type(Type::String, Type::Table(vec![]))
            .usage("Parse text as agprefs and create a table.")
            .category(Category::Formats)]
    }

    fn run(
        &mut self,
        name: &str,
        call: &EvaluatedCall,
        input: &nu_protocol::Value,
    ) -> Result<nu_protocol::Value, LabeledError> {
        if name != CALL_SIGNATURE {
            return Err(LabeledError {
                label: "Plugin call with wrong name signature".into(),
                msg: "the signature used to call the plugin does not match any name in the plugin signature vector".into(),
                span: Some(call.head),
            });
        }

        let span = input.span();
        let input = input.as_string()?;
        let agp = agprefs::Agpref::parse(&input).unwrap();
        // let head = call.head;
        let mut record = Record::new();
        record.push(agp.name, agpref_to_value(agp.values, span));

        Ok(nu_protocol::Value::record(record, span))
    }
}

fn agpref_to_value(agp: agprefs::Value, span: nu_protocol::span::Span) -> nu_protocol::Value {
    use agprefs::Value;
    match agp {
        Value::Unit => nu_protocol::Value::nothing(span),
        Value::Bool(b) => nu_protocol::Value::bool(b, span),
        Value::Int(i) => nu_protocol::Value::int(i, span),
        Value::Float(f) => nu_protocol::Value::float(f, span),
        Value::String(s) => nu_protocol::Value::string(s, span),
        Value::Values(v) => nu_protocol::Value::list(
            v.into_iter().map(|x| agpref_to_value(x, span)).collect(),
            span,
        ),
        Value::Struct(r) => nu_protocol::Value::record(
            {
                let (cols, rows) = r
                    .into_iter()
                    .map(|(k, v)| (k.to_string(), agpref_to_value(v, span)))
                    .unzip();
                Record { cols, vals: rows }
            },
            span,
        ),
    }
}
