<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Get all employee data</description>
   <name>All Employees</name>
   <tag></tag>
   <elementGuidId>b04a5d21-2744-46e3-bc86-4e1374640ff5</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://dummy.restapiexample.com/api/v1/employees</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()



assertThat(response.getResponseText()).contains('&quot;id&quot;:&quot;1&quot;,&quot;employee_name&quot;:&quot;paul999&quot;,&quot;employee_salary&quot;:&quot;1123&quot;,&quot;employee_age&quot;:&quot;23&quot;,&quot;profile_image&quot;:&quot;&quot;')


assertThat(response.getResponseText()).contains('&quot;id&quot;:&quot;1&quot;')


WS.verifyElementPropertyValue(response, '[3958].employee_name', 'Bappy')


WS.verifyElementPropertyValue(response, '[3958].id', '111281')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
